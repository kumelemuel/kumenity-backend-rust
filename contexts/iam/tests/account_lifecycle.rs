//! Characterization of the account lifecycle through the inbound use-case
//! ports, wired with the real production adapters (in-memory repository,
//! Argon2 hasher, JWT service), the same way the composition root does it.
//!
//! The HTTP API cannot reach the happy path because the verification code is
//! random and only delivered out of band. Here the repository stands in for
//! that channel: it is the only place the test looks past the ports.

use iam::{
    application::{
        commands::{
            authenticate_account::{
                authenticate_account::AuthenticateAccount, command::AuthenticateAccountCommand,
            },
            register_account::{RegisterAccountCommand, RegisterAccountHandler},
            verify_account::{
                VerifyAccountCommand, VerifyAccountError, VerifyAccountHandler,
                VerifyAccountResponse,
            },
        },
        ports::{
            inbound::{
                authenticate_account_use_case::AuthenticateAccountUseCase,
                identify_account_use_case::IdentifyAccountUseCase,
                register_account_use_case::RegisterAccountUseCase,
                verify_account_use_case::VerifyAccountUseCase,
            },
            outbound::{
                access_token_service::AccessTokenService, account_repository::AccountRepository,
                dtos::access_token_payloads::AccessTokenVerdict,
                password_hashing_service::PasswordHashingService,
            },
        },
        queries::identify_account::{
            identify_account::IdentifyAccount, query::IdentifyAccountQuery,
        },
    },
    domain::model::account::account_status::AccountStatus,
    infrastructure::{
        persistence::in_memory::in_memory_account::InMemoryAccountAdapter,
        security::argon2_password_hashing::Argon2PasswordHashingAdapter,
        services::jwt_access_token::JwtAccessTokenAdapter,
    },
};
use shared::{Failure, domain::value_object::ValueObject};
use std::sync::Arc;

const PASSWORD: &str = "s3cret-password";

struct Iam {
    repository: Arc<InMemoryAccountAdapter>,
    jwt: Arc<JwtAccessTokenAdapter>,
    register: RegisterAccountHandler,
    verify: VerifyAccountHandler,
    authenticate: AuthenticateAccount,
    identify: IdentifyAccount,
}

fn iam() -> Iam {
    let repository = Arc::new(InMemoryAccountAdapter::new());
    let hasher = Arc::new(Argon2PasswordHashingAdapter::new());
    let decoy_hash = hasher.hash("lifecycle-decoy").expect("decoy hash");
    let jwt = Arc::new(JwtAccessTokenAdapter::new("lifecycle-secret".into(), 3600));
    Iam {
        register: RegisterAccountHandler::new(repository.clone(), hasher.clone()),
        verify: VerifyAccountHandler::new(repository.clone()),
        authenticate: AuthenticateAccount::new(repository.clone(), hasher, jwt.clone(), decoy_hash),
        identify: IdentifyAccount::new(repository.clone()),
        repository,
        jwt,
    }
}

impl Iam {
    fn register_ana(&self) -> String {
        self.register
            .execute(RegisterAccountCommand {
                username: "ana".into(),
                email: "ana@example.com".into(),
                password: PASSWORD.into(),
            })
            .expect("register")
            .id
    }

    /// Reads the verification code that would be sent to the user.
    fn delivered_code(&self, email: &str) -> u32 {
        match *self
            .repository
            .find_by_email(email)
            .expect("repository answers")
            .expect("account was registered")
            .status()
        {
            AccountStatus::Registered { code_validation } => *code_validation.value(),
            other => panic!("account is not pending verification: {other:?}"),
        }
    }

    fn verify_with(&self, code: u32) -> Result<VerifyAccountResponse, Failure<VerifyAccountError>> {
        self.verify.execute(VerifyAccountCommand {
            email: "ana@example.com".into(),
            code,
        })
    }

    fn sign_in(&self, password: &str) -> Result<String, shared::error::SystemError> {
        self.authenticate
            .execute(AuthenticateAccountCommand {
                username: "ana".into(),
                password: password.into(),
            })
            .map(|r| r.token)
    }

    fn status_of(&self, identify: &str) -> String {
        self.identify
            .execute(IdentifyAccountQuery {
                identify: identify.into(),
            })
            .expect("account can be identified")
            .status
    }
}

#[test]
fn verified_account_signs_in_with_a_token_whose_subject_is_the_account_id() {
    let iam = iam();
    let account_id = iam.register_ana();
    let code = iam.delivered_code("ana@example.com");

    iam.verify_with(code).expect("verify");
    let token = iam.sign_in(PASSWORD).expect("sign in");

    assert_eq!(iam.status_of("ana"), "active");
    assert_eq!(
        iam.jwt.verify(&token).unwrap(),
        AccessTokenVerdict::Valid { account_id }
    );
}

#[test]
fn verification_code_is_six_digits() {
    let iam = iam();
    iam.register_ana();

    let code = iam.delivered_code("ana@example.com");

    assert!((100_000..=999_999).contains(&code), "code: {code}");
}

#[test]
fn verifying_twice_fails_with_invalid_verification() {
    let iam = iam();
    iam.register_ana();
    let code = iam.delivered_code("ana@example.com");
    iam.verify_with(code).unwrap();

    let error = iam.verify_with(code).unwrap_err();

    assert!(
        matches!(
            error,
            Failure::Expected(VerifyAccountError::InvalidVerification(_))
        ),
        "{error:?}"
    );
    assert_eq!(iam.status_of("ana"), "active");
}

#[test]
fn wrong_code_keeps_account_registered_and_unable_to_sign_in() {
    let iam = iam();
    iam.register_ana();
    let code = iam.delivered_code("ana@example.com");
    let wrong = if code == 100_000 { 100_001 } else { 100_000 };

    let error = iam.verify_with(wrong).unwrap_err();

    assert!(
        matches!(
            error,
            Failure::Expected(VerifyAccountError::InvalidVerification(_))
        ),
        "{error:?}"
    );
    assert_eq!(iam.status_of("ana"), "registered");
    assert_eq!(
        iam.sign_in(PASSWORD).unwrap_err().code(),
        "IAM_CANNOT_AUTHENTICATE"
    );
    // The original code still works after a failed attempt.
    iam.verify_with(code).expect("verify with the right code");
}

#[test]
fn verified_account_with_wrong_password_fails_with_login_failed() {
    let iam = iam();
    iam.register_ana();
    iam.verify_with(iam.delivered_code("ana@example.com"))
        .unwrap();

    let error = iam.sign_in("wrong-password").unwrap_err();

    assert_eq!(error.code(), "IAM_LOGIN_FAILED");
}

#[test]
fn password_is_not_stored_in_plain_text() {
    let iam = iam();
    iam.register_ana();

    let account = iam
        .repository
        .find_by_email("ana@example.com")
        .unwrap()
        .unwrap();

    assert_ne!(account.password().value(), PASSWORD);
}

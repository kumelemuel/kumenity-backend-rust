use super::{AuthenticateAccountCommand, AuthenticateAccountError, AuthenticateAccountResponse};
use crate::{
    application::{
        errors::{cannot_authenticate::CannotAuthenticate, login_failed::LoginFailed},
        ports::{
            inbound::authenticate_account_use_case::AuthenticateAccountUseCase,
            outbound::{
                access_token_service::AccessTokenService, account_repository::AccountRepository,
                password_hashing_service::PasswordHashingService,
            },
        },
    },
    domain::errors::CannotAuthenticate as AccountCannotAuthenticate,
};
use shared::{Failure, domain::value_object::ValueObject};
use std::sync::Arc;

/// Orchestrates (HEX-7): loads the account, checks the password, asks the
/// aggregate whether it can sign in and issues the token.
///
/// An unknown username and a wrong password are the same answer,
/// `LoginFailed`, and cost the same (D6).
pub struct AuthenticateAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
    access_tokens: Arc<dyn AccessTokenService>,
    decoy_hash: String,
}

impl AuthenticateAccountHandler {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
        access_tokens: Arc<dyn AccessTokenService>,
        decoy_hash: String,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
            access_tokens,
            decoy_hash,
        }
    }
}

impl AuthenticateAccountUseCase for AuthenticateAccountHandler {
    fn execute(
        &self,
        cmd: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, Failure<AuthenticateAccountError>> {
        // `?` propagates every `Fault` as tree B.
        let Some(account) = self
            .account_repository
            .find_by_username(cmd.username.as_str())?
        else {
            // Pay the same hashing cost as a wrong password, so the response
            // time does not reveal whether the username exists. Only the
            // outcome is discarded (`black_box` keeps the optimizer from
            // dropping the call); a `Fault` propagates like the real one.
            std::hint::black_box(
                self.password_hasher
                    .verify(cmd.password.as_str(), &self.decoy_hash)?,
            );
            return Err(Failure::expected(LoginFailed));
        };

        if !self
            .password_hasher
            .verify(cmd.password.as_str(), account.password().value())?
        {
            return Err(Failure::expected(LoginFailed));
        }

        // Domain error → flat leaf of the contract (ERR-2).
        account
            .authenticate()
            .map_err(|AccountCannotAuthenticate| Failure::expected(CannotAuthenticate))?;

        let token = self
            .access_tokens
            .issue(&account.id().as_uuid().to_string())?;

        Ok(AuthenticateAccountResponse { token })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::{
            ports::outbound::{
                access_token_service::AccessTokenService,
                dtos::access_token_payloads::AccessTokenVerdict,
                password_hashing_service::PasswordHashingService,
            },
            testing::{
                AccessTokenOperation, FakeAccessTokenAdapter, FakeAccountAdapter,
                FixedPasswordHashingAdapter, PasswordHashingOperation, SpyPasswordHashingAdapter,
            },
        },
        domain::model::account::{
            account::Account, account_factory::AccountFactory, account_id::AccountId,
        },
    };
    const PASSWORD: &str = "dummy-password";

    fn hash_of(password: &str) -> String {
        FixedPasswordHashingAdapter::new()
            .hash(password)
            .expect("invariante: el doble sano no falla")
    }

    /// `dummy`, with `PASSWORD` hashed by `FixedPasswordHashingAdapter`.
    fn dummy_account(status: &str) -> Account {
        AccountFactory::reconstitute(
            AccountId::generate().as_uuid().to_string(),
            "dummy".to_string(),
            "dummy@example.com".to_string(),
            hash_of(PASSWORD),
            status.to_string(),
        )
        .expect("invariante: cuenta de prueba válida")
    }

    fn command(username: &str, password: &str) -> AuthenticateAccountCommand {
        AuthenticateAccountCommand {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    #[test]
    fn verifies_against_the_decoy_hash_when_account_does_not_exist() {
        let hasher = Arc::new(SpyPasswordHashingAdapter::new());
        let decoy_hash = hash_of("decoy");
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            hasher.clone(),
            Arc::new(FakeAccessTokenAdapter::new()),
            decoy_hash.clone(),
        );

        let _ = use_case.execute(command("ghost", "any_password"));

        assert_eq!(
            hasher.verify_calls(),
            vec![("any_password".to_string(), decoy_hash)]
        );
    }

    #[test]
    fn fails_with_login_failed_when_account_does_not_exist() {
        // The decoy is the hash of the password sent, so its verification
        // answers `true`: the handler has to discard the outcome.
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            Arc::new(SpyPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("any_password"),
        );

        let result = use_case.execute(command("ghost", "any_password"));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(AuthenticateAccountError::LoginFailed(
                    LoginFailed
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::failing()),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", PASSWORD));

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_verify_fails() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account("active")])),
            Arc::new(
                FixedPasswordHashingAdapter::new().failing_on(PasswordHashingOperation::Verify),
            ),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", PASSWORD));

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_decoy_verify_fails() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            Arc::new(
                FixedPasswordHashingAdapter::new().failing_on(PasswordHashingOperation::Verify),
            ),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("ghost", PASSWORD));

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_issue_fails() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account("active")])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new().failing_on(AccessTokenOperation::Issue)),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", PASSWORD));

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_incorrect_password() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account("active")])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", "wrong_password"));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(AuthenticateAccountError::LoginFailed(
                    LoginFailed
                )))
            ),
            "{result:?}"
        );
    }

    /// The password before the status: an inactive account with a wrong
    /// password is `LoginFailed`, so the answer does not reveal that the
    /// account exists and is not active (D6).
    #[test]
    fn fails_with_login_failed_when_an_inactive_account_gets_a_wrong_password() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                "registered:123123",
            )])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", "wrong_password"));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(AuthenticateAccountError::LoginFailed(
                    LoginFailed
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_account_cannot_authenticate() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                "registered:123123",
            )])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", PASSWORD));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(
                    AuthenticateAccountError::CannotAuthenticate(CannotAuthenticate)
                ))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn authenticate_account_successfully_returns_token() {
        let account = dummy_account("active");
        let account_id = account.id().as_uuid().to_string();
        let tokens = Arc::new(FakeAccessTokenAdapter::new());
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([account])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            tokens.clone(),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("dummy", PASSWORD));

        let token = result.expect("Expected token").token;
        assert_eq!(
            tokens.verify(&token).unwrap(),
            AccessTokenVerdict::Valid { account_id }
        );
    }
}

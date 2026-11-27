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
    domain::{
        errors::{CannotAuthenticate as AccountCannotAuthenticate, UsernameError},
        model::account::{hashed_password::HashedPassword, username::Username},
    },
};
use shared::Failure;
use std::sync::Arc;

/// Orchestrates (HEX-7): loads the account, checks the password, asks the
/// aggregate whether it can sign in and issues the token.
///
/// An unknown or malformed username and a wrong password are the same answer,
/// `LoginFailed`, and cost the same (D6).
pub struct AuthenticateAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
    access_tokens: Arc<dyn AccessTokenService>,
    decoy_hash: HashedPassword,
}

impl AuthenticateAccountHandler {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
        access_tokens: Arc<dyn AccessTokenService>,
        decoy_hash: HashedPassword,
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
        // Looked up normalized, as the registration stores it (D10). A
        // malformed username cannot belong to any account: it takes the same
        // path as an unknown one. `?` propagates every `Fault` as tree B.
        let found = match Username::new(cmd.username) {
            Ok(username) => self.account_repository.find_by_username(username.value())?,
            Err(UsernameError::Invalid) => None,
        };
        let Some(account) = found else {
            // Pay the same hashing cost as a wrong password, so the response
            // time does not reveal whether the username exists. Only the
            // outcome is discarded (`black_box` keeps the optimizer from
            // dropping the call); a `Fault` propagates like the real one.
            std::hint::black_box(
                self.password_hasher
                    .verify(cmd.password.as_str(), self.decoy_hash.value())?,
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

        let token = self.access_tokens.issue(account.id().as_str())?;

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
            Account, account_factory::AccountFactory, account_id::AccountId,
            account_status::AccountStatus, email::Email, hashed_password::HashedPassword,
            username::Username,
        },
    };
    const PASSWORD: &str = "dummy-password";

    fn hash_of(password: &str) -> HashedPassword {
        FixedPasswordHashingAdapter::new()
            .hash(password)
            .expect("invariante: el doble sano no falla")
    }

    /// `dummy`, with `PASSWORD` hashed by `FixedPasswordHashingAdapter`.
    fn dummy_account(status: AccountStatus) -> Account {
        Account::restore(
            AccountId::new("acc-1".to_string()),
            Username::new("dummy".to_string()).expect("invariante: username de prueba válido"),
            Email::new("dummy@example.com".to_string())
                .expect("invariante: email de prueba válido"),
            hash_of(PASSWORD),
            status,
        )
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
            vec![("any_password".to_string(), decoy_hash.value().clone())]
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

    /// D6 also for a malformed username ("a b" cannot be a `Username`): it
    /// costs the same as a wrong password.
    #[test]
    fn verifies_against_the_decoy_hash_when_identifier_is_malformed() {
        let hasher = Arc::new(SpyPasswordHashingAdapter::new());
        let decoy_hash = hash_of("decoy");
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
            )])),
            hasher.clone(),
            Arc::new(FakeAccessTokenAdapter::new()),
            decoy_hash.clone(),
        );

        let _ = use_case.execute(command("a b", PASSWORD));

        assert_eq!(
            hasher.verify_calls(),
            vec![(PASSWORD.to_string(), decoy_hash.value().clone())]
        );
    }

    /// A malformed username is answered like an unknown one: no new 400. The
    /// decoy is the hash of the password sent, so the handler has to discard
    /// its outcome.
    #[test]
    fn fails_with_login_failed_when_identifier_is_malformed() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
            )])),
            Arc::new(SpyPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of(PASSWORD),
        );

        let result = use_case.execute(command("a b", PASSWORD));

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

    /// D10: the username is looked up normalized, as the registration stores it.
    #[test]
    fn authenticates_with_surrounding_spaces_in_the_username() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
            )])),
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FakeAccessTokenAdapter::new()),
            hash_of("decoy"),
        );

        let result = use_case.execute(command("  dummy  ", PASSWORD));

        assert!(result.is_ok(), "{result:?}");
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
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
            )])),
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
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
            )])),
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
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountStatus::Active,
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

    /// The password before the status: an inactive account with a wrong
    /// password is `LoginFailed`, so the answer does not reveal that the
    /// account exists and is not active (D6).
    #[test]
    fn fails_with_login_failed_when_an_inactive_account_gets_a_wrong_password() {
        let use_case = AuthenticateAccountHandler::new(
            Arc::new(FakeAccountAdapter::containing([dummy_account(
                AccountFactory::pending(123_123),
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
                AccountFactory::pending(123_123),
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
        let account = dummy_account(AccountStatus::Active);
        let account_id = account.id().as_str().to_owned();
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

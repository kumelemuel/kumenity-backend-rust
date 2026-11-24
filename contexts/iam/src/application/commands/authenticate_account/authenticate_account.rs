use crate::application::{
    commands::authenticate_account::{
        command::AuthenticateAccountCommand, response::AuthenticateAccountResponse,
    },
    errors::authenticate_account::AuthenticateAccountError,
    ports::{
        inbound::authenticate_account_use_case::AuthenticateAccountUseCase,
        outbound::{
            account_repository::AccountRepository, password_hasher::PasswordHashingService,
            token_generator::TokenGeneratorPort,
        },
    },
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct AuthenticateAccount {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
    token_generator: Arc<dyn TokenGeneratorPort>,
    decoy_hash: String,
}

impl AuthenticateAccount {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
        token_generator: Arc<dyn TokenGeneratorPort>,
        decoy_hash: String,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
            token_generator,
            decoy_hash,
        }
    }
}

impl AuthenticateAccountUseCase for AuthenticateAccount {
    fn execute(
        &self,
        cmd: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, SystemError> {
        let Some(account) = self
            .account_repository
            .find_by_username(cmd.username.as_str())
            .map_err(SystemError::Unexpected)?
        else {
            // Pay the same hashing cost as a wrong password, so the response
            // time does not reveal whether the username exists. Only the
            // outcome is discarded (`black_box` keeps the optimizer from
            // dropping the call); a `Fault` propagates like the real one.
            std::hint::black_box(
                self.password_hasher
                    .verify(cmd.password.as_str(), &self.decoy_hash)
                    .map_err(SystemError::Unexpected)?,
            );
            return Err(AuthenticateAccountError::LoginFailed.into());
        };

        if !self
            .password_hasher
            .verify(cmd.password.as_str(), account.password().value())
            .map_err(SystemError::Unexpected)?
        {
            return Err(AuthenticateAccountError::LoginFailed.into());
        }

        if !account.can_authenticate() {
            return Err(AuthenticateAccountError::CannotAuthenticate.into());
        }

        let token = self
            .token_generator
            .generate(&account.id().as_uuid().to_string())?;

        Ok(AuthenticateAccountResponse { token })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::authenticate_account::{
                authenticate_account::AuthenticateAccount, command::AuthenticateAccountCommand,
            },
            errors::error_codes::{IAM_CANNOT_AUTHENTICATE, IAM_LOGIN_FAILED},
            ports::{
                inbound::authenticate_account_use_case::AuthenticateAccountUseCase,
                outbound::{
                    password_hasher::{PasswordHashingService, test_utils::FakePasswordHasher},
                    token_generator::test_utils::FakeTokenGenerator,
                },
            },
            testing::FakeAccountAdapter,
        },
        domain::model::account::account_factory::AccountFactory,
    };
    use shared::{Fault, error::SystemError};
    use std::sync::{Arc, Mutex};

    const DECOY_HASH: &str = "decoy-hash";

    /// Records every `verify` call. It answers `true` so
    /// `fails_with_login_failed_when_account_does_not_exist` proves the handler
    /// ignores the outcome of the decoy verification.
    #[derive(Default)]
    struct SpyPasswordHashingAdapter {
        verify_calls: Mutex<Vec<(String, String)>>,
    }

    impl SpyPasswordHashingAdapter {
        fn verify_calls(&self) -> Vec<(String, String)> {
            self.verify_calls
                .lock()
                .expect("invariante: mutex sin envenenar")
                .clone()
        }
    }

    impl PasswordHashingService for SpyPasswordHashingAdapter {
        fn hash(&self, _password: &str) -> Result<String, Fault> {
            Ok(String::from("unused"))
        }

        fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
            self.verify_calls
                .lock()
                .expect("invariante: mutex sin envenenar")
                .push((password.to_string(), hashed_password.to_string()));
            Ok(true)
        }
    }

    #[test]
    fn verifies_against_the_decoy_hash_when_account_does_not_exist() {
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher = Arc::new(SpyPasswordHashingAdapter::default());
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccount::new(
            repo,
            hasher.clone(),
            token_generator,
            DECOY_HASH.to_string(),
        );

        let _ = use_case.execute(AuthenticateAccountCommand {
            username: "ghost".to_string(),
            password: "any_password".to_string(),
        });

        assert_eq!(
            hasher.verify_calls(),
            vec![("any_password".to_string(), DECOY_HASH.to_string())]
        );
    }

    #[test]
    fn fails_with_login_failed_when_account_does_not_exist() {
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher = Arc::new(SpyPasswordHashingAdapter::default());
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case =
            AuthenticateAccount::new(repo, hasher, token_generator, DECOY_HASH.to_string());

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "ghost".to_string(),
            password: "any_password".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_LOGIN_FAILED);
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo = Arc::new(FakeAccountAdapter::failing());
        let use_case = AuthenticateAccount::new(
            repo,
            Arc::new(FakePasswordHasher),
            Arc::new(FakeTokenGenerator),
            DECOY_HASH.to_string(),
        );

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "any_password".to_string(),
        });

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_incorrect_password() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case =
            AuthenticateAccount::new(repo, hasher, token_generator, DECOY_HASH.to_string());

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "wrong_password".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_LOGIN_FAILED);
    }

    #[test]
    fn fails_when_account_cannot_authenticate() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case =
            AuthenticateAccount::new(repo, hasher, token_generator, DECOY_HASH.to_string());

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_CANNOT_AUTHENTICATE);
    }

    #[test]
    fn authenticate_account_successfully_returns_token() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("active".to_string()),
        ]));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case =
            AuthenticateAccount::new(repo, hasher, token_generator, DECOY_HASH.to_string());

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "valid_token");
    }
}

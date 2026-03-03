use crate::{
    application::{
        commands::authenticate_account::AuthenticateAccount,
        errors::authenticate_account::AuthenticateAccountError,
        ports::{
            inbound::account_authentication::AccountAuthenticationPort,
            outbound::{
                account_repository::AccountRepositoryPort, password_hasher::PasswordHasherPort,
                token_generator::TokenGeneratorPort,
            },
        },
        results::account_authenticated::AccountAuthenticated,
    },
    domain::{aggregates::Account, errors::AccountError},
};
use shared::error::SystemError;
use std::sync::Arc;

pub struct AuthenticateAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_generator: Arc<dyn TokenGeneratorPort>,
}

impl AuthenticateAccountUseCase {
    pub fn new(
        account_repository: Arc<dyn AccountRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_generator: Arc<dyn TokenGeneratorPort>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
            token_generator,
        }
    }
}

impl AccountAuthenticationPort for AuthenticateAccountUseCase {
    fn execute(&self, cmd: AuthenticateAccount) -> Result<AccountAuthenticated, SystemError> {
        let account: Option<Account>;

        let existing_username = self
            .account_repository
            .find_by_username(cmd.username.as_str());
        if existing_username.is_some() {
            account = existing_username;
        } else {
            return Err(AccountError::AccountNotFound.into());
        }
        let account = account.unwrap();

        if !self
            .password_hasher
            .verify(cmd.password.as_str(), &account.password())
        {
            return Err(AuthenticateAccountError::LoginFailed.into());
        }

        if !account.can_authenticate() {
            return Err(AuthenticateAccountError::CannotAuthenticate.into());
        }

        let token = self
            .token_generator
            .generate(&account.id().as_uuid().to_string())?;

        Ok(AccountAuthenticated { token })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::authenticate_account::AuthenticateAccount,
            errors::error_codes::{IAM_CANNOT_AUTHENTICATE, IAM_LOGIN_FAILED},
            ports::{
                inbound::account_authentication::AccountAuthenticationPort,
                outbound::{
                    account_repository::test_utils::FakeAccountRepository,
                    password_hasher::test_utils::FakePasswordHasher,
                    token_generator::test_utils::FakeTokenGenerator,
                },
            },
            use_cases::authenticate_account::AuthenticateAccountUseCase,
        },
        domain::{errors::error_codes::IAM_ACCOUNT_NOT_FOUND, value_objects::HashedPassword},
    };
    use std::sync::Arc;

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    #[test]
    fn fails_with_incorrect_password() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: "wrong_password".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_LOGIN_FAILED);
    }

    #[test]
    fn fails_when_account_cannot_authenticate() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_CANNOT_AUTHENTICATE);
    }

    #[test]
    fn authenticate_account_successfully_returns_token() {
        let repo = Arc::new(FakeAccountRepository::active_with_existing_username(
            "dummy",
        ));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "valid_token");
    }
}

use crate::{
    application::{
        commands::authenticate_account::{
            command::AuthenticateAccountCommand, response::AuthenticateAccountResponse,
        },
        errors::authenticate_account::AuthenticateAccountError,
        ports::{
            inbound::authenticate_account_use_case::AuthenticateAccountUseCase,
            outbound::{
                account_repository::AccountRepositoryPort, password_hasher::PasswordHasherPort,
                token_generator::TokenGeneratorPort,
            },
        },
    },
    domain::{errors::AccountError, model::account::account::Account},
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct AuthenticateAccount {
    account_repository: Arc<dyn AccountRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_generator: Arc<dyn TokenGeneratorPort>,
}

impl AuthenticateAccount {
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

impl AuthenticateAccountUseCase for AuthenticateAccount {
    fn execute(
        &self,
        cmd: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, SystemError> {
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
            .verify(cmd.password.as_str(), &account.password().value())
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
                    account_repository::test_utils::FakeAccountRepository,
                    password_hasher::test_utils::FakePasswordHasher,
                    token_generator::test_utils::FakeTokenGenerator,
                },
            },
        },
        domain::errors::error_codes::IAM_ACCOUNT_NOT_FOUND,
    };
    use std::sync::Arc;

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccount::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    #[test]
    fn fails_with_incorrect_password() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccount::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccountCommand {
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

        let use_case = AuthenticateAccount::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
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

        let use_case = AuthenticateAccount::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccountCommand {
            username: "dummy".to_string(),
            password: "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "valid_token");
    }
}

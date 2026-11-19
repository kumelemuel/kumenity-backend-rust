use crate::{
    application::{
        commands::register_account::{
            command::RegisterAccountCommand, response::RegisterAccountResponse,
        },
        ports::{
            inbound::register_account_use_case::RegisterAccountUseCase,
            outbound::{
                account_repository::AccountRepositoryPort, password_hasher::PasswordHasherPort,
            },
        },
    },
    domain::{errors::AccountError, model::account::account_factory::AccountFactory},
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct RegisterAccount {
    account_repository: Arc<dyn AccountRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
}

impl RegisterAccount {
    pub fn new(
        account_repository: Arc<dyn AccountRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
        }
    }
}

impl RegisterAccountUseCase for RegisterAccount {
    fn execute(&self, cmd: RegisterAccountCommand) -> Result<RegisterAccountResponse, SystemError> {
        let existing_email = self.account_repository.find_by_email(cmd.email.as_str());
        if existing_email.is_some() {
            return Err(AccountError::EmailAlreadyExists.into());
        }

        let existing_username = self
            .account_repository
            .find_by_username(cmd.username.as_str());
        if existing_username.is_some() {
            return Err(AccountError::UsernameAlreadyExists.into());
        }

        let hashed_password = self.password_hasher.hash(&cmd.password);
        let account =
            AccountFactory::create(cmd.username, cmd.email, &cmd.password, hashed_password)?;

        self.account_repository.save(&account)?;

        dbg!(&account);

        Ok(RegisterAccountResponse {
            id: account.id().as_uuid().to_string(),
            username: account.username().value().to_owned(),
            email: account.email().value().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::register_account::{
                command::RegisterAccountCommand, register_account::RegisterAccount,
            },
            errors::error_codes::IAM_ACCOUNT_REPOSITORY_ERROR,
            ports::{
                inbound::register_account_use_case::RegisterAccountUseCase,
                outbound::{
                    account_repository::test_utils::FakeAccountRepository,
                    password_hasher::test_utils::FakePasswordHasher,
                },
            },
        },
        domain::errors::error_codes::{
            IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
            IAM_INVALID_EMAIL, IAM_INVALID_USERNAME,
        },
    };
    use std::sync::Arc;

    fn valid_input() -> RegisterAccountCommand {
        RegisterAccountCommand {
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        }
    }

    #[test]
    fn register_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_username_is_invalid() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let input = RegisterAccountCommand {
            username: "".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_INVALID_USERNAME);
    }

    #[test]
    fn fails_when_email_is_invalid() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let input = RegisterAccountCommand {
            username: "john_doe".to_string(),
            email: "invalid-email".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_INVALID_EMAIL);
    }

    #[test]
    fn fails_when_repository_fails() {
        let repo = Arc::new(FakeAccountRepository::fail());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let result = use_case.execute(valid_input());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_REPOSITORY_ERROR);
    }

    #[test]
    fn fails_when_username_already_exists() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("john_doe"));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let result = use_case.execute(valid_input());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_USERNAME_ALREADY_EXISTS);
    }

    #[test]
    fn fails_when_email_already_exists() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email(
            "john@example.com",
        ));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccount::new(repo, hasher);

        let result = use_case.execute(valid_input());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_EMAIL_ALREADY_EXISTS);
    }
}

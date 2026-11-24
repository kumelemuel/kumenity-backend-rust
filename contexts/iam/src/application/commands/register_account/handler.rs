use crate::{
    application::{
        commands::register_account::{
            command::RegisterAccountCommand, response::RegisterAccountResponse,
        },
        ports::{
            inbound::register_account_use_case::RegisterAccountUseCase,
            outbound::{
                account_repository::AccountRepository,
                password_hashing_service::PasswordHashingService,
            },
        },
    },
    domain::{
        errors::{AccountCreationError, AccountError},
        model::account::account_factory::AccountFactory,
    },
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct RegisterAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
}

impl RegisterAccountHandler {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
        }
    }
}

impl RegisterAccountUseCase for RegisterAccountHandler {
    fn execute(&self, cmd: RegisterAccountCommand) -> Result<RegisterAccountResponse, SystemError> {
        let existing_email = self
            .account_repository
            .find_by_email(cmd.email.as_str())
            .map_err(SystemError::Unexpected)?;
        if existing_email.is_some() {
            return Err(AccountError::EmailAlreadyExists.into());
        }

        let existing_username = self
            .account_repository
            .find_by_username(cmd.username.as_str())
            .map_err(SystemError::Unexpected)?;
        if existing_username.is_some() {
            return Err(AccountError::UsernameAlreadyExists.into());
        }

        let hashed_password = self
            .password_hasher
            .hash(&cmd.password)
            .map_err(SystemError::Unexpected)?;
        let account =
            AccountFactory::create(cmd.username, cmd.email, &cmd.password, hashed_password)
                .map_err(|error| -> SystemError {
                    match error {
                        AccountCreationError::Password(e) => e.into(),
                        AccountCreationError::Username(e) => e.into(),
                        AccountCreationError::Email(e) => e.into(),
                        AccountCreationError::HashedPassword(e) => e.into(),
                    }
                })?;

        self.account_repository
            .save(&account)
            .map_err(SystemError::Unexpected)?;

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
                command::RegisterAccountCommand, handler::RegisterAccountHandler,
            },
            ports::inbound::register_account_use_case::RegisterAccountUseCase,
            testing::{
                AccountOperation, FakeAccountAdapter, FixedPasswordHashingAdapter,
                PasswordHashingOperation,
            },
        },
        domain::{
            errors::error_codes::{
                IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
                IAM_INVALID_EMAIL, IAM_INVALID_USERNAME,
            },
            model::account::{account::Account, account_factory::AccountFactory},
        },
    };
    use shared::error::SystemError;
    use std::sync::Arc;

    fn valid_input() -> RegisterAccountCommand {
        RegisterAccountCommand {
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        }
    }

    const HASH: &str =
        "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012";

    fn registered(username: &str, email: &str) -> Account {
        AccountFactory::create(
            username.to_string(),
            email.to_string(),
            "password123456789",
            HASH.to_string(),
        )
        .expect("invariante: cuenta de prueba válida")
    }

    #[test]
    fn register_account_successfully() {
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher = Arc::new(FixedPasswordHashingAdapter::new());

        let use_case = RegisterAccountHandler::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_username_is_invalid() {
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher = Arc::new(FixedPasswordHashingAdapter::new());

        let use_case = RegisterAccountHandler::new(repo, hasher);

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
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher = Arc::new(FixedPasswordHashingAdapter::new());

        let use_case = RegisterAccountHandler::new(repo, hasher);

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
    fn fails_with_unexpected_when_find_by_email_fails() {
        let repo = Arc::new(FakeAccountAdapter::failing());
        let use_case =
            RegisterAccountHandler::new(repo, Arc::new(FixedPasswordHashingAdapter::new()));

        let result = use_case.execute(valid_input());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo =
            Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByUsername));
        let use_case =
            RegisterAccountHandler::new(repo, Arc::new(FixedPasswordHashingAdapter::new()));

        let result = use_case.execute(valid_input());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::Save));
        let use_case =
            RegisterAccountHandler::new(repo, Arc::new(FixedPasswordHashingAdapter::new()));

        let result = use_case.execute(valid_input());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_hash_fails() {
        let repo = Arc::new(FakeAccountAdapter::empty());
        let hasher =
            Arc::new(FixedPasswordHashingAdapter::new().failing_on(PasswordHashingOperation::Hash));
        let use_case = RegisterAccountHandler::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_username_already_exists() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "john_doe",
            "other@example.com",
        )]));
        let hasher = Arc::new(FixedPasswordHashingAdapter::new());

        let use_case = RegisterAccountHandler::new(repo, hasher);

        let result = use_case.execute(valid_input());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_USERNAME_ALREADY_EXISTS);
    }

    #[test]
    fn fails_when_email_already_exists() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "other_user",
            "john@example.com",
        )]));
        let hasher = Arc::new(FixedPasswordHashingAdapter::new());

        let use_case = RegisterAccountHandler::new(repo, hasher);

        let result = use_case.execute(valid_input());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_EMAIL_ALREADY_EXISTS);
    }
}

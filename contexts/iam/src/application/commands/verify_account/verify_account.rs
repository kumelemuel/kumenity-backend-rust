use crate::{
    application::{
        commands::verify_account::command::VerifyAccountCommand,
        ports::{
            inbound::verify_account_use_case::VerifyAccountUseCase,
            outbound::account_repository::AccountRepository,
        },
    },
    domain::{errors::AccountError, model::account::code_validation::CodeValidation},
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct VerifyAccount {
    account_repository: Arc<dyn AccountRepository>,
}

impl VerifyAccount {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }
}

impl VerifyAccountUseCase for VerifyAccount {
    fn execute(&self, cmd: VerifyAccountCommand) -> Result<(), SystemError> {
        let Some(mut account) = self
            .account_repository
            .find_by_email(cmd.email.as_str())
            .map_err(SystemError::Unexpected)?
        else {
            return Err(AccountError::AccountNotFound.into());
        };

        let code_validation = CodeValidation::new(cmd.code)?;
        account.confirm_registration(code_validation)?;

        self.account_repository
            .save(&account)
            .map_err(SystemError::Unexpected)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::verify_account::{
                command::VerifyAccountCommand, verify_account::VerifyAccount,
            },
            ports::inbound::verify_account_use_case::VerifyAccountUseCase,
            testing::{AccountOperation, FakeAccountAdapter},
        },
        domain::{
            errors::error_codes::{IAM_ACCOUNT_INVALID_VERIFICATION, IAM_ACCOUNT_NOT_FOUND},
            model::account::account_factory::AccountFactory,
        },
    };
    use shared::error::SystemError;
    use std::sync::Arc;

    /// `dummy_account` está pendiente de verificar con el código 123123.
    fn valid_command() -> VerifyAccountCommand {
        VerifyAccountCommand {
            email: "john@example.com".to_string(),
            code: 123123,
        }
    }

    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let repo = Arc::new(FakeAccountAdapter::failing());
        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(valid_command());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(
            FakeAccountAdapter::containing([AccountFactory::dummy_account()])
                .failing_on(AccountOperation::Save),
        );
        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(valid_command());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn verify_account_successfully() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));

        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(VerifyAccountCommand {
            email: "dummy@example.com".to_string(),
            code: 123123,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeAccountAdapter::empty());

        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(VerifyAccountCommand {
            email: "not-exists@example.com".to_string(),
            code: 123123,
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    #[test]
    fn fails_when_code_not_match() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));

        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(VerifyAccountCommand {
            email: "dummy@example.com".to_string(),
            code: 123111,
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_INVALID_VERIFICATION);
    }
}

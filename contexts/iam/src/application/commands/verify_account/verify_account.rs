use crate::{
    application::{
        commands::verify_account::command::VerifyAccountCommand,
        ports::{
            inbound::verify_account_use_case::VerifyAccountUseCase,
            outbound::account_repository::AccountRepositoryPort,
        },
    },
    domain::{errors::AccountError, model::account::code_validation::CodeValidation},
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct VerifyAccount {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl VerifyAccount {
    pub fn new(account_repository: Arc<dyn AccountRepositoryPort>) -> Self {
        Self { account_repository }
    }
}

impl VerifyAccountUseCase for VerifyAccount {
    fn execute(&self, cmd: VerifyAccountCommand) -> Result<(), SystemError> {
        let account = self.account_repository.find_by_email(cmd.email.as_str());
        if account.is_none() {
            return Err(AccountError::AccountNotFound.into());
        }
        let mut account = account.unwrap();

        let code_validation = CodeValidation::new(cmd.code)?;
        account.confirm_registration(code_validation)?;

        self.account_repository.save(&account)?;

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
            ports::{
                inbound::verify_account_use_case::VerifyAccountUseCase,
                outbound::account_repository::test_utils::FakeAccountRepository,
            },
        },
        domain::errors::error_codes::{IAM_ACCOUNT_INVALID_VERIFICATION, IAM_ACCOUNT_NOT_FOUND},
    };
    use std::sync::Arc;

    #[test]
    fn verify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email(
            "dummy@example.com",
        ));

        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(VerifyAccountCommand {
            email: "dummy@example.com".to_string(),
            code: 123123,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

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
        let repo = Arc::new(FakeAccountRepository::with_existing_email(
            "dummy@example.com",
        ));

        let use_case = VerifyAccount::new(repo);

        let result = use_case.execute(VerifyAccountCommand {
            email: "dummy@example.com".to_string(),
            code: 123111,
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_INVALID_VERIFICATION);
    }
}

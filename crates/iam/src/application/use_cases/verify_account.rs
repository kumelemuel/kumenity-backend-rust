use crate::{
    application::{
        commands::verify_account::VerifyAccount,
        ports::{
            inbound::account_verification::AccountVerificationPort,
            outbound::account_repository::AccountRepositoryPort,
        },
    },
    domain::{errors::AccountError, value_objects::CodeValidation},
};
use shared::error::SystemError;
use std::sync::Arc;

pub struct VerifyAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl VerifyAccountUseCase {
    pub fn new(account_repository: Arc<dyn AccountRepositoryPort>) -> Self {
        Self { account_repository }
    }
}

impl AccountVerificationPort for VerifyAccountUseCase {
    fn execute(&self, cmd: VerifyAccount) -> Result<bool, SystemError> {
        let account = self.account_repository.find_by_email(cmd.email.as_str());
        if account.is_none() {
            return Err(AccountError::AccountNotFound.into());
        }
        let mut account = account.unwrap();
        let code_validation = CodeValidation::new(cmd.code)?;
        account.confirm_registration(code_validation)?;

        self.account_repository.save(&account)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::verify_account::VerifyAccount,
            ports::{
                inbound::account_verification::AccountVerificationPort,
                outbound::account_repository::test_utils::FakeAccountRepository,
            },
            use_cases::verify_account::VerifyAccountUseCase,
        },
        domain::errors::error_codes::{
            IAM_ACCOUNT_NOT_FOUND, IAM_INVALID_ACCOUNT_STATUS_TRANSITION,
        },
    };
    use std::sync::Arc;

    #[test]
    fn verify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email(
            "dummy@example.com",
        ));

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "dummy@example.com".to_string(),
            code: 123123,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
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

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "dummy@example.com".to_string(),
            code: 123111,
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_INVALID_ACCOUNT_STATUS_TRANSITION);
    }
}

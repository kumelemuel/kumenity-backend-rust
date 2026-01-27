use std::sync::Arc;
use shared::application::common_application_error::CommonApplicationError;
use crate::application::commands::verify_account::VerifyAccount;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::inbound::account_verification::AccountVerificationPort;
use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::domain::value_objects::{CodeValidation, AccountStatus};

pub struct VerifyAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl VerifyAccountUseCase {
    pub fn new(account_repository: Arc<dyn AccountRepositoryPort>) -> Self {
        Self { account_repository }
    }
}

impl AccountVerificationPort for VerifyAccountUseCase {
    fn execute(&self, cmd: VerifyAccount) -> Result<bool, ApplicationError> {
        let account = self.account_repository.find_by_email(cmd.email.as_str());
        if account.is_none() {
            return Err(ApplicationError::UserNotFound);
        }
        let mut account = account.unwrap();
        let code_validation = CodeValidation::new(cmd.code).map_err(|_| ApplicationError::InvalidCodeValidation)?;
        if account.status().ne(&AccountStatus::Registered { code_validation }) {
            return Err(ApplicationError::ActivationFailed);
        }

        account.activate().expect("error while activating user");
        self.account_repository
            .save(&account)
            .map_err(|_| CommonApplicationError::Infrastructure)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::application::commands::verify_account::VerifyAccount;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::inbound::account_verification::AccountVerificationPort;
    use crate::application::use_cases::verify_account::VerifyAccountUseCase;
    use crate::application::ports::outbound::account_repository::test_utils::FakeAccountRepository;

    #[test]
    fn verify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("dummy@example.com"));

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "dummy@example.com".to_string(),
            code: 123123
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "not-exists@example.com".to_string(),
            code: 123123
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn fails_when_code_not_match() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("dummy@example.com"));

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "dummy@example.com".to_string(),
            code: 123111
        });

        assert!(matches!(result, Err(ApplicationError::ActivationFailed)));
    }
}
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
    use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
    use crate::application::use_cases::verify_account::VerifyAccountUseCase;
    use crate::domain::aggregates::Account;
    use crate::domain::value_objects::{CodeValidation, Email, HashedPassword, AccountId, AccountStatus, Username};

    struct FakeAccountRepository {
        should_fail: bool,
        existing_username: Option<String>,
        existing_email: Option<String>,
    }

    impl FakeAccountRepository {
        fn success() -> Self {
            Self {
                should_fail: false,
                existing_username: None,
                existing_email: None,
            }
        }

        fn with_existing_email(email: &str) -> Self {
            Self {
                should_fail: false,
                existing_username: None,
                existing_email: Some(email.to_string()),
            }
        }
    }

    impl AccountRepositoryPort for FakeAccountRepository {
        fn find_by_username(&self, username: &str) -> Option<Account> {
            self.existing_username
                .as_ref()
                .filter(|u| u.as_str() == username)
                .map(|_| dummy_account())
        }

        fn find_by_email(&self, email: &str) -> Option<Account> {
            self.existing_email
                .as_ref()
                .filter(|e| e.as_str() == email)
                .map(|_| dummy_account())
        }

        fn save(&self, _user: &Account) -> Result<(), String> {
            if self.should_fail {
                Err("Unexpected error".to_string())
            } else {
                Ok(())
            }
        }
    }

    fn dummy_account() -> Account {
        Account::reconstitute(
            AccountId::generate(),
            Username::new("dummy".to_string()).unwrap(),
            Email::new("dummy@example.com").unwrap(),
            HashedPassword::dummy(),
            AccountStatus::Registered { code_validation: CodeValidation::new(123123).unwrap() },
        )
    }

    #[test]
    fn verify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("john@example.com"));

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "john@example.com".to_string(),
            code: 123123
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "john@example.com".to_string(),
            code: 123123
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn fails_when_code_not_match() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("john@example.com"));

        let use_case = VerifyAccountUseCase::new(repo);

        let result = use_case.execute(VerifyAccount {
            email: "john@example.com".to_string(),
            code: 123111
        });

        assert!(matches!(result, Err(ApplicationError::ActivationFailed)));
    }
}
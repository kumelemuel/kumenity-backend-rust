use std::sync::Arc;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::application::commands::identify_account::IdentifyAccount;
use crate::application::ports::inbound::account_identification::AccountIdentificationPort;
use crate::application::results::account_identified::AccountIdentified;
use crate::domain::aggregates::Account;

pub struct IdentifyAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl IdentifyAccountUseCase {
    pub fn new(
        account_repository: Arc<dyn AccountRepositoryPort>,
    ) -> Self {
        Self {
            account_repository,
        }
    }
}

impl AccountIdentificationPort for IdentifyAccountUseCase {
    fn execute(&self, cmd: IdentifyAccount) -> Result<AccountIdentified, ApplicationError> {
        let mut account: Option<Account> = None;
        let existing_email = self.account_repository.find_by_email(cmd.identify.as_str());
        if existing_email.is_none() {
            let existing_username = self
                .account_repository
                .find_by_username(cmd.identify.as_str());
            if existing_username.is_some() {
                account = existing_username;
            }
        } else {
            account = existing_email;
        }

        if account.is_none() {
            return Err(ApplicationError::UserNotFound);
        }
        let account = account.unwrap();

        Ok(AccountIdentified {
            username: account.username().as_str().to_owned(),
            status: account.status().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::commands::identify_account::IdentifyAccount;
    use crate::application::ports::inbound::account_identification::AccountIdentificationPort;
    use crate::application::ports::outbound::account_repository::tests::FakeAccountRepository;
    use crate::application::use_cases::identify_account::IdentifyAccountUseCase;

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = IdentifyAccountUseCase::new(repo);

        let result = use_case.execute(IdentifyAccount {
            identify: "not-exists@example.com".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn identify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("dummy@example.com"));

        let use_case = IdentifyAccountUseCase::new(repo);

        let result = use_case.execute(IdentifyAccount {
            identify: "dummy@example.com".to_string(),
        });

        assert!(result.is_ok());
    }


}

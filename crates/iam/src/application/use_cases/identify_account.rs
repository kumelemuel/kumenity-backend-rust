use crate::{
    application::{
        commands::identify_account::IdentifyAccount,
        ports::{
            inbound::account_identification::AccountIdentificationPort,
            outbound::account_repository::AccountRepositoryPort,
        },
        results::account_identified::AccountIdentified,
    },
    domain::{aggregates::Account, errors::AccountError},
};
use shared::error::SystemError;
use std::sync::Arc;

pub struct IdentifyAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl IdentifyAccountUseCase {
    pub fn new(account_repository: Arc<dyn AccountRepositoryPort>) -> Self {
        Self { account_repository }
    }
}

impl AccountIdentificationPort for IdentifyAccountUseCase {
    fn execute(&self, cmd: IdentifyAccount) -> Result<AccountIdentified, SystemError> {
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
            return Err(AccountError::AccountNotFound.into());
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
    use crate::{
        application::{
            commands::identify_account::IdentifyAccount,
            ports::{
                inbound::account_identification::AccountIdentificationPort,
                outbound::account_repository::test_utils::FakeAccountRepository,
            },
            use_cases::identify_account::IdentifyAccountUseCase,
        },
        domain::errors::error_codes::IAM_ACCOUNT_NOT_FOUND,
    };
    use std::sync::Arc;

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = IdentifyAccountUseCase::new(repo);

        let result = use_case.execute(IdentifyAccount {
            identify: "not-exists@example.com".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    #[test]
    fn identify_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email(
            "dummy@example.com",
        ));

        let use_case = IdentifyAccountUseCase::new(repo);

        let result = use_case.execute(IdentifyAccount {
            identify: "dummy@example.com".to_string(),
        });

        assert!(result.is_ok());
    }
}

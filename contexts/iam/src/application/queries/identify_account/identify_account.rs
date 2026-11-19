use crate::{
    application::{
        ports::{
            inbound::identify_account_use_case::IdentifyAccountUseCase,
            outbound::account_repository::AccountRepositoryPort,
        },
        queries::identify_account::{
            query::IdentifyAccountQuery, response::IdentifyAccountResponse,
        },
    },
    domain::{errors::AccountError, model::account::account::Account},
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct IdentifyAccount {
    account_repository: Arc<dyn AccountRepositoryPort>,
}

impl IdentifyAccount {
    pub fn new(account_repository: Arc<dyn AccountRepositoryPort>) -> Self {
        Self { account_repository }
    }
}

impl IdentifyAccountUseCase for IdentifyAccount {
    fn execute(&self, cmd: IdentifyAccountQuery) -> Result<IdentifyAccountResponse, SystemError> {
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

        Ok(IdentifyAccountResponse {
            username: account.username().value().to_owned(),
            status: account.status().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            ports::{
                inbound::identify_account_use_case::IdentifyAccountUseCase,
                outbound::account_repository::test_utils::FakeAccountRepository,
            },
            queries::identify_account::{
                identify_account::IdentifyAccount, query::IdentifyAccountQuery,
            },
        },
        domain::errors::error_codes::IAM_ACCOUNT_NOT_FOUND,
    };
    use std::sync::Arc;

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());

        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
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

        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "dummy@example.com".to_string(),
        });

        assert!(result.is_ok());
    }
}

use crate::{
    application::{
        ports::{
            inbound::identify_account_use_case::IdentifyAccountUseCase,
            outbound::account_repository::AccountRepository,
        },
        queries::identify_account::{
            query::IdentifyAccountQuery, response::IdentifyAccountResponse,
        },
    },
    domain::errors::AccountError,
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct IdentifyAccount {
    account_repository: Arc<dyn AccountRepository>,
}

impl IdentifyAccount {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }
}

impl IdentifyAccountUseCase for IdentifyAccount {
    fn execute(&self, cmd: IdentifyAccountQuery) -> Result<IdentifyAccountResponse, SystemError> {
        let by_email = self
            .account_repository
            .find_by_email(cmd.identify.as_str())
            .map_err(SystemError::Unexpected)?;
        let found = match by_email {
            Some(account) => Some(account),
            None => self
                .account_repository
                .find_by_username(cmd.identify.as_str())
                .map_err(SystemError::Unexpected)?,
        };
        let Some(account) = found else {
            return Err(AccountError::AccountNotFound.into());
        };

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
            ports::inbound::identify_account_use_case::IdentifyAccountUseCase,
            queries::identify_account::{
                identify_account::IdentifyAccount, query::IdentifyAccountQuery,
            },
            testing::{AccountOperation, FakeAccountAdapter},
        },
        domain::{
            errors::error_codes::IAM_ACCOUNT_NOT_FOUND,
            model::account::account_factory::AccountFactory,
        },
    };
    use shared::error::SystemError;
    use std::sync::Arc;

    fn query() -> IdentifyAccountQuery {
        IdentifyAccountQuery {
            identify: "dummy".to_string(),
        }
    }

    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let repo = Arc::new(FakeAccountAdapter::failing());
        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(query());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo =
            Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByUsername));
        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(query());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountAdapter::empty());

        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "not-exists@example.com".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    #[test]
    fn identify_account_successfully() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));

        let use_case = IdentifyAccount::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "dummy@example.com".to_string(),
        });

        assert!(result.is_ok());
    }
}

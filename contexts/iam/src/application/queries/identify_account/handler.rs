use crate::{
    application::{
        ports::{
            inbound::identify_account_use_case::IdentifyAccountUseCase,
            outbound::account_repository::AccountRepository,
        },
        queries::identify_account::{IdentifyAccountQuery, IdentifyAccountResponse},
    },
    domain::{
        errors::{AccountError, EmailError, UsernameError},
        model::account::{email::Email, username::Username},
    },
};
use shared::{domain::value_object::ValueObject, error::SystemError};
use std::sync::Arc;

pub struct IdentifyAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl IdentifyAccountHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }
}

impl IdentifyAccountUseCase for IdentifyAccountHandler {
    fn execute(&self, cmd: IdentifyAccountQuery) -> Result<IdentifyAccountResponse, SystemError> {
        // Looked up normalized, as the registration stores it (D10): by email
        // if the identifier is one, then by username. One that is neither
        // cannot belong to any account: `NotFound`, never a new 400.
        let by_email = match Email::new(cmd.identify.clone()) {
            Ok(email) => self
                .account_repository
                .find_by_email(email.value())
                .map_err(SystemError::Unexpected)?,
            Err(EmailError::Invalid) => None,
        };
        let found = match by_email {
            Some(account) => Some(account),
            None => match Username::new(cmd.identify) {
                Ok(username) => self
                    .account_repository
                    .find_by_username(username.value())
                    .map_err(SystemError::Unexpected)?,
                Err(UsernameError::Invalid) => None,
            },
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
            queries::identify_account::{IdentifyAccountHandler, IdentifyAccountQuery},
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

    /// An email-shaped identifier, so the lookup reaches `find_by_email`;
    /// only that operation fails, so a swallowed `Fault` cannot hide behind
    /// the `find_by_username` that follows.
    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let repo = Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByEmail));
        let use_case = IdentifyAccountHandler::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "dummy@example.com".to_string(),
        });

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo =
            Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByUsername));
        let use_case = IdentifyAccountHandler::new(repo);

        let result = use_case.execute(query());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountAdapter::empty());

        let use_case = IdentifyAccountHandler::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "not-exists@example.com".to_string(),
        });

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    /// D10: looked up normalized, as the registration stores it.
    #[test]
    fn identifies_with_an_email_that_differs_only_in_case_or_surrounding_spaces() {
        for identify in ["Dummy@Example.COM", "  dummy@example.com  "] {
            let use_case = IdentifyAccountHandler::new(registered());

            let result = use_case.execute(IdentifyAccountQuery {
                identify: identify.to_string(),
            });

            assert_eq!(
                result.map(|found| found.username).ok().as_deref(),
                Some("dummy"),
                "{identify:?}"
            );
        }
    }

    /// D10, with the username.
    #[test]
    fn identifies_with_surrounding_spaces_in_the_username() {
        let use_case = IdentifyAccountHandler::new(registered());

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "  dummy  ".to_string(),
        });

        assert_eq!(
            result.map(|found| found.username).ok().as_deref(),
            Some("dummy")
        );
    }

    /// Neither an email nor a username: answered like an unknown account.
    #[test]
    fn fails_with_not_found_when_identifier_is_malformed() {
        let use_case = IdentifyAccountHandler::new(registered());

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "a b".to_string(),
        });

        let err = result.expect_err("Expected error");
        assert_eq!(err.code(), IAM_ACCOUNT_NOT_FOUND);
    }

    fn registered() -> Arc<FakeAccountAdapter> {
        Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]))
    }

    #[test]
    fn identify_account_successfully() {
        let repo = Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]));

        let use_case = IdentifyAccountHandler::new(repo);

        let result = use_case.execute(IdentifyAccountQuery {
            identify: "dummy@example.com".to_string(),
        });

        assert!(result.is_ok());
    }
}

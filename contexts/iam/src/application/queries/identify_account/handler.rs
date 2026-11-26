use super::{IdentifyAccountError, IdentifyAccountQuery, IdentifyAccountResponse};
use crate::{
    application::{
        errors::not_found::NotFound,
        ports::{
            inbound::identify_account_use_case::IdentifyAccountUseCase,
            outbound::account_repository::AccountRepository,
        },
    },
    domain::{
        errors::{EmailError, UsernameError},
        model::account::{email::Email, username::Username},
    },
};
use shared::{Failure, domain::value_object::ValueObject};
use std::sync::Arc;

/// Orchestrates (HEX-7): builds the identifier, loads the account and maps
/// it to the response.
pub struct IdentifyAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl IdentifyAccountHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }
}

impl IdentifyAccountUseCase for IdentifyAccountHandler {
    fn execute(
        &self,
        query: IdentifyAccountQuery,
    ) -> Result<IdentifyAccountResponse, Failure<IdentifyAccountError>> {
        // Looked up normalized, as the registration stores it (D10): by email
        // if the identifier is one, then by username. One that is neither
        // cannot belong to any account: `NotFound`, never a new 400.
        // `?` propagates the `Fault` of each lookup as tree B.
        let by_email = match Email::new(query.identify.clone()) {
            Ok(email) => self.account_repository.find_by_email(email.value())?,
            Err(EmailError::Invalid) => None,
        };
        let found = match by_email {
            Some(account) => Some(account),
            None => match Username::new(query.identify) {
                Ok(username) => self.account_repository.find_by_username(username.value())?,
                Err(UsernameError::Invalid) => None,
            },
        };
        // `find_*` answers an Option; this use case requires the account
        // (ERR-1).
        let Some(account) = found else {
            return Err(Failure::expected(NotFound {
                resource: "account",
            }));
        };

        Ok(IdentifyAccountResponse {
            username: account.username().value().to_owned(),
            status: account.status().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::testing::{AccountOperation, FakeAccountAdapter},
        domain::model::account::account_factory::AccountFactory,
    };

    type Outcome = Result<IdentifyAccountResponse, Failure<IdentifyAccountError>>;

    fn identify_with(repo: Arc<FakeAccountAdapter>, identify: &str) -> Outcome {
        IdentifyAccountHandler::new(repo).execute(IdentifyAccountQuery {
            identify: identify.to_string(),
        })
    }

    /// `dummy` / `dummy@example.com`, pending verification.
    fn registered() -> Arc<FakeAccountAdapter> {
        Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account_with_status("registered:123123".to_string()),
        ]))
    }

    fn is_account_not_found(result: &Outcome) -> bool {
        matches!(
            result,
            Err(Failure::Expected(IdentifyAccountError::NotFound(
                NotFound {
                    resource: "account"
                }
            )))
        )
    }

    #[test]
    fn identifies_by_email_with_username_and_status() {
        let result = identify_with(registered(), "dummy@example.com");

        let found = result.expect("identified");
        assert_eq!(found.username, "dummy");
        assert_eq!(found.status, "registered");
    }

    #[test]
    fn identifies_by_username_with_username_and_status() {
        let result = identify_with(registered(), "dummy");

        let found = result.expect("identified");
        assert_eq!(found.username, "dummy");
        assert_eq!(found.status, "registered");
    }

    #[test]
    fn fails_with_not_found_for_unknown_identifier() {
        for identify in ["not-exists@example.com", "ghost"] {
            let result = identify_with(registered(), identify);

            assert!(is_account_not_found(&result), "{identify:?}: {result:?}");
        }
    }

    /// D10: looked up normalized, as the registration stores it.
    #[test]
    fn identifies_with_an_email_that_differs_only_in_case_or_surrounding_spaces() {
        for identify in ["Dummy@Example.COM", "  dummy@example.com  "] {
            let result = identify_with(registered(), identify);

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
        let result = identify_with(registered(), "  dummy  ");

        assert_eq!(
            result.map(|found| found.username).ok().as_deref(),
            Some("dummy")
        );
    }

    /// Neither an email nor a username: answered like an unknown account.
    #[test]
    fn fails_with_not_found_when_identifier_is_malformed() {
        let result = identify_with(registered(), "a b");

        assert!(is_account_not_found(&result), "{result:?}");
    }

    /// An email-shaped identifier, so the lookup reaches `find_by_email`;
    /// only that operation fails, so a swallowed `Fault` cannot hide behind
    /// the `find_by_username` that follows.
    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let repo = Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByEmail));

        let result = identify_with(repo, "dummy@example.com");

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo =
            Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByUsername));

        let result = identify_with(repo, "dummy");

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }
}

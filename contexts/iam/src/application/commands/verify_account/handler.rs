use super::{VerifyAccountCommand, VerifyAccountError};
use crate::{
    application::{
        errors::{
            invalid_verification::InvalidVerification, not_found::NotFound, validation::Validation,
        },
        ports::{
            inbound::verify_account_use_case::VerifyAccountUseCase,
            outbound::account_repository::AccountRepository,
        },
    },
    domain::{
        errors::{AccountError, CodeValidationError},
        model::account::code_validation::CodeValidation,
    },
};
use shared::{Failure, Fault, domain::value_object::ValueObject};
use std::sync::Arc;

/// Orchestrates (HEX-7): loads the account, builds the code, asks the
/// aggregate to confirm the registration and saves it.
///
/// The order is today's: the account first, then the code. An unknown email
/// is `NotFound` even with a code out of range.
pub struct VerifyAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl VerifyAccountHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }
}

impl VerifyAccountUseCase for VerifyAccountHandler {
    fn execute(&self, cmd: VerifyAccountCommand) -> Result<(), Failure<VerifyAccountError>> {
        // `find_*` answers an Option; this use case requires the account
        // (ERR-1). `?` propagates the `Fault` as tree B.
        let Some(mut account) = self.account_repository.find_by_email(cmd.email.as_str())? else {
            return Err(Failure::expected(NotFound {
                resource: "account",
            }));
        };

        // Domain errors → flat leaf of the contract, by variant (ERR-2).
        let code = CodeValidation::new(cmd.code).map_err(|error| match error {
            CodeValidationError::Invalid => Failure::expected(Validation { field: "code" }),
        })?;

        account
            .confirm_registration(code)
            .map_err(|error| match error {
                // A wrong code and an account already verified are the same
                // answer (D1).
                AccountError::InvalidVerification => Failure::expected(InvalidVerification),
                // `confirm_registration` never answers it: a bug, tree B.
                AccountError::AccountNotFound => {
                    Failure::Unexpected(Fault::new("confirm_registration answered AccountNotFound"))
                }
            })?;

        self.account_repository.save(&account)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::testing::{AccountOperation, FakeAccountAdapter},
        domain::model::account::account_factory::AccountFactory,
    };

    type Outcome = Result<(), Failure<VerifyAccountError>>;

    /// `dummy_account` is pending verification with the code 123123.
    fn valid_command() -> VerifyAccountCommand {
        with("john@example.com", 123123)
    }

    fn with(email: &str, code: u32) -> VerifyAccountCommand {
        VerifyAccountCommand {
            email: email.to_string(),
            code,
        }
    }

    fn verify_with(repo: Arc<FakeAccountAdapter>, command: VerifyAccountCommand) -> Outcome {
        VerifyAccountHandler::new(repo).execute(command)
    }

    fn pending() -> Arc<FakeAccountAdapter> {
        Arc::new(FakeAccountAdapter::containing([
            AccountFactory::dummy_account(),
        ]))
    }

    #[test]
    fn verify_account_successfully() {
        let repo = pending();

        let result = verify_with(repo.clone(), valid_command());

        assert!(result.is_ok(), "{result:?}");
        let saved = repo
            .find_by_email("john@example.com")
            .expect("invariante: el Fake responde")
            .expect("invariante: la cuenta está guardada");
        assert_eq!(saved.status().as_str(), "active");
    }

    #[test]
    fn fails_with_not_found_when_email_is_unknown() {
        let result = verify_with(
            Arc::new(FakeAccountAdapter::empty()),
            with("not-exists@example.com", 123123),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccountError::NotFound(NotFound {
                    resource: "account"
                })))
            ),
            "{result:?}"
        );
    }

    /// Account before code: today's order (see the handler).
    #[test]
    fn fails_with_not_found_before_checking_the_code() {
        let result = verify_with(
            Arc::new(FakeAccountAdapter::empty()),
            with("not-exists@example.com", 99_999),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccountError::NotFound(_)))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_code_is_out_of_range() {
        for code in [0, 99_999, 1_000_000] {
            let result = verify_with(pending(), with("john@example.com", code));

            assert!(
                matches!(
                    result,
                    Err(Failure::Expected(VerifyAccountError::Validation(
                        Validation { field: "code" }
                    )))
                ),
                "{code}: {result:?}"
            );
        }
    }

    #[test]
    fn fails_with_invalid_verification_when_code_does_not_match() {
        let result = verify_with(pending(), with("john@example.com", 123111));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccountError::InvalidVerification(
                    InvalidVerification
                )))
            ),
            "{result:?}"
        );
    }

    /// D1: verifying an account already verified is `InvalidVerification`.
    #[test]
    fn fails_with_invalid_verification_when_account_is_already_verified() {
        let repo = pending();
        verify_with(repo.clone(), valid_command()).expect("first verification");

        let result = verify_with(repo, valid_command());

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccountError::InvalidVerification(
                    InvalidVerification
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let result = verify_with(Arc::new(FakeAccountAdapter::failing()), valid_command());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(
            FakeAccountAdapter::containing([AccountFactory::dummy_account()])
                .failing_on(AccountOperation::Save),
        );

        let result = verify_with(repo, valid_command());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }
}

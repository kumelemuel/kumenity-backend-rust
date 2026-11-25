use super::{RegisterAccountCommand, RegisterAccountError, RegisterAccountResponse};
use crate::{
    application::{
        errors::{conflict::Conflict, validation::Validation},
        ports::{
            inbound::register_account_use_case::RegisterAccountUseCase,
            outbound::{
                account_repository::AccountRepository,
                password_hashing_service::PasswordHashingService,
            },
        },
    },
    domain::{errors::AccountCreationError, model::account::account_factory::AccountFactory},
};
use shared::{Failure, Fault, domain::value_object::ValueObject};
use std::sync::Arc;

/// Orchestrates (HEX-7): checks for conflicts, hashes, asks the factory for
/// the account, saves it and maps it.
///
/// The order is today's, fixed by `app/tests/`: email conflict, username
/// conflict, hash, and then the factory's rules (password, username, email).
/// R12 changes the conflict checks to the normalized values.
pub struct RegisterAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
}

impl RegisterAccountHandler {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
        }
    }
}

impl RegisterAccountUseCase for RegisterAccountHandler {
    fn execute(
        &self,
        cmd: RegisterAccountCommand,
    ) -> Result<RegisterAccountResponse, Failure<RegisterAccountError>> {
        // `find_*` answers an Option; this use case decides that existing is
        // a conflict (ERR-1). `?` propagates the `Fault` as tree B.
        if self
            .account_repository
            .find_by_email(cmd.email.as_str())?
            .is_some()
        {
            return Err(Failure::expected(Conflict { resource: "email" }));
        }

        if self
            .account_repository
            .find_by_username(cmd.username.as_str())?
            .is_some()
        {
            return Err(Failure::expected(Conflict {
                resource: "username",
            }));
        }

        let hashed_password = self.password_hasher.hash(&cmd.password)?;

        // Domain errors → flat leaf of the contract, by variant (ERR-2).
        let draft =
            AccountFactory::draft(cmd.username, cmd.email, &cmd.password).map_err(|error| {
                match error {
                    AccountCreationError::Password(_) => {
                        Failure::expected(Validation { field: "password" })
                    }
                    AccountCreationError::Username(_) => {
                        Failure::expected(Validation { field: "username" })
                    }
                    AccountCreationError::Email(_) => {
                        Failure::expected(Validation { field: "email" })
                    }
                }
            })?;

        // A hash that is not a `HashedPassword` is a bug of the hasher, not
        // something the client can fix: tree B.
        let account = AccountFactory::create(draft, hashed_password)
            .map_err(|_| Fault::new("password hasher returned a malformed hash"))?;

        self.account_repository.save(&account)?;

        Ok(RegisterAccountResponse {
            id: account.id().as_uuid().to_string(),
            username: account.username().value().to_owned(),
            email: account.email().value().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::testing::{
            AccountOperation, FakeAccountAdapter, FixedPasswordHashingAdapter,
            PasswordHashingOperation,
        },
        domain::model::account::account::Account,
    };

    type Outcome = Result<RegisterAccountResponse, Failure<RegisterAccountError>>;

    fn valid_input() -> RegisterAccountCommand {
        with("john_doe", "john@example.com", "password123456789")
    }

    fn with(username: &str, email: &str, password: &str) -> RegisterAccountCommand {
        RegisterAccountCommand {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    const HASH: &str =
        "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012";

    fn registered(username: &str, email: &str) -> Account {
        let draft =
            AccountFactory::draft(username.to_string(), email.to_string(), "password123456789")
                .expect("invariante: borrador de prueba válido");
        AccountFactory::create(draft, HASH.to_string()).expect("invariante: hash de prueba válido")
    }

    fn register_with(repo: Arc<FakeAccountAdapter>, input: RegisterAccountCommand) -> Outcome {
        RegisterAccountHandler::new(repo, Arc::new(FixedPasswordHashingAdapter::new()))
            .execute(input)
    }

    #[test]
    fn register_account_successfully() {
        let result = register_with(Arc::new(FakeAccountAdapter::empty()), valid_input());

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn fails_with_validation_on_invalid_username() {
        let result = register_with(
            Arc::new(FakeAccountAdapter::empty()),
            with("", "john@example.com", "password123456789"),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(RegisterAccountError::Validation(
                    Validation { field: "username" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_validation_on_invalid_email() {
        let result = register_with(
            Arc::new(FakeAccountAdapter::empty()),
            with("john_doe", "invalid-email", "password123456789"),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(RegisterAccountError::Validation(
                    Validation { field: "email" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_password_is_too_short() {
        let result = register_with(
            Arc::new(FakeAccountAdapter::empty()),
            with("john_doe", "john@example.com", "1234567"),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(RegisterAccountError::Validation(
                    Validation { field: "password" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_conflict_on_taken_email() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "other_user",
            "john@example.com",
        )]));

        let result = register_with(repo, valid_input());

        assert!(
            matches!(
                result,
                Err(Failure::Expected(RegisterAccountError::Conflict(
                    Conflict { resource: "email" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_conflict_on_taken_username() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "john_doe",
            "other@example.com",
        )]));

        let result = register_with(repo, valid_input());

        assert!(
            matches!(
                result,
                Err(Failure::Expected(RegisterAccountError::Conflict(
                    Conflict {
                        resource: "username"
                    }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_email_fails() {
        let result = register_with(Arc::new(FakeAccountAdapter::failing()), valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_find_by_username_fails() {
        let repo =
            Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::FindByUsername));

        let result = register_with(repo, valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(FakeAccountAdapter::empty().failing_on(AccountOperation::Save));

        let result = register_with(repo, valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_hash_fails() {
        let hasher =
            Arc::new(FixedPasswordHashingAdapter::new().failing_on(PasswordHashingOperation::Hash));
        let use_case = RegisterAccountHandler::new(Arc::new(FakeAccountAdapter::empty()), hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    /// Answers a hash that is not a `HashedPassword`. It breaks the
    /// `PasswordHashingService` contract on purpose, so it does not run the
    /// battery: it stands for an adapter with a bug.
    struct StubPasswordHashingAdapter;

    impl PasswordHashingService for StubPasswordHashingAdapter {
        fn hash(&self, _password: &str) -> Result<String, Fault> {
            Ok("not-a-hash".to_string())
        }

        fn verify(&self, _password: &str, _hashed_password: &str) -> Result<bool, Fault> {
            Ok(false)
        }
    }

    #[test]
    fn fails_with_unexpected_when_the_hasher_returns_a_malformed_hash() {
        let use_case = RegisterAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            Arc::new(StubPasswordHashingAdapter),
        );

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }
}

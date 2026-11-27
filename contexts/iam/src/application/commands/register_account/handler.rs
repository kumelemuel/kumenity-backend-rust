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
    domain::{
        errors::AccountCreationError,
        model::account::{account_factory::AccountFactory, account_id::AccountId},
    },
};
use shared::{Failure, Fault, IdGenerator, domain::value_object::ValueObject};
use std::sync::Arc;

/// Orchestrates (HEX-7): asks the factory for a draft, checks for conflicts
/// with its normalized values, hashes, asks the factory for the account,
/// saves it and maps it.
///
/// The order is fixed by `app/tests/`: the factory's rules (password,
/// username, email), then the email conflict, then the username conflict.
pub struct RegisterAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
    password_hasher: Arc<dyn PasswordHashingService>,
    id_generator: Arc<dyn IdGenerator>,
}

impl RegisterAccountHandler {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHashingService>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
            id_generator,
        }
    }
}

impl RegisterAccountUseCase for RegisterAccountHandler {
    fn execute(
        &self,
        cmd: RegisterAccountCommand,
    ) -> Result<RegisterAccountResponse, Failure<RegisterAccountError>> {
        // Domain errors → flat leaf of the contract, by variant (ERR-2). The
        // draft comes first: password, then format, then conflict.
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

        // Uniqueness is looked up with the normalized values of the draft,
        // the same ones the account is saved with (HEX-7.1). `find_*` answers
        // an Option; this use case decides that existing is a conflict
        // (ERR-1). `?` propagates the `Fault` as tree B.
        if self
            .account_repository
            .find_by_email(draft.email().value())?
            .is_some()
        {
            return Err(Failure::expected(Conflict { resource: "email" }));
        }

        if self
            .account_repository
            .find_by_username(draft.username().value())?
            .is_some()
        {
            return Err(Failure::expected(Conflict {
                resource: "username",
            }));
        }

        let hashed_password = self.password_hasher.hash(&cmd.password)?;

        // A hash that is not a `HashedPassword` is a bug of the hasher, not
        // something the client can fix: tree B.
        let id = AccountId::new(self.id_generator.next_id());
        let account = AccountFactory::create(id, draft, hashed_password)
            .map_err(|_| Fault::new("password hasher returned a malformed hash"))?;

        self.account_repository.save(&account)?;

        Ok(RegisterAccountResponse {
            id: account.id().as_str().to_owned(),
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
            AccountOperation, FakeAccountAdapter, FixedIdGeneratorAdapter,
            FixedPasswordHashingAdapter, PasswordHashingOperation,
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

    const ID: &str = "acc-1";

    const HASH: &str =
        "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012";

    fn registered(username: &str, email: &str) -> Account {
        let draft =
            AccountFactory::draft(username.to_string(), email.to_string(), "password123456789")
                .expect("invariante: borrador de prueba válido");
        AccountFactory::create(
            AccountId::new(format!("id-{username}")),
            draft,
            HASH.to_string(),
        )
        .expect("invariante: hash de prueba válido")
    }

    fn register_with(repo: Arc<FakeAccountAdapter>, input: RegisterAccountCommand) -> Outcome {
        RegisterAccountHandler::new(
            repo,
            Arc::new(FixedPasswordHashingAdapter::new()),
            Arc::new(FixedIdGeneratorAdapter(ID)),
        )
        .execute(input)
    }

    #[test]
    fn register_account_successfully() {
        let result = register_with(Arc::new(FakeAccountAdapter::empty()), valid_input());

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn registers_with_the_generated_id() {
        let repo = Arc::new(FakeAccountAdapter::empty());

        let response = register_with(repo.clone(), valid_input()).expect("registro válido");

        assert_eq!(response.id, ID);
        let saved = repo
            .find_by_email("john@example.com")
            .expect("invariante: el Fake vacío no falla")
            .expect("la cuenta quedó guardada");
        assert_eq!(saved.id().as_str(), ID);
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
    fn fails_with_conflict_when_email_differs_only_in_case() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "other_user",
            "john@example.com",
        )]));

        let result = register_with(
            repo,
            with("john_doe", " John@Example.COM ", "password123456789"),
        );

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
    fn fails_with_conflict_when_username_differs_only_in_surrounding_spaces() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "john_doe",
            "other@example.com",
        )]));

        let result = register_with(
            repo,
            with("  john_doe ", "john@example.com", "password123456789"),
        );

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

    /// Format before conflict: `register_rejects_invalid_username_before_checking_taken_email`.
    #[test]
    fn fails_with_validation_on_invalid_username_before_checking_taken_email() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "other_user",
            "john@example.com",
        )]));

        let result = register_with(repo, with("ab", "john@example.com", "password123456789"));

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

    /// Password before conflict, as in
    /// `register_validates_password_before_username_and_email`.
    #[test]
    fn fails_with_validation_on_short_password_before_checking_taken_email() {
        let repo = Arc::new(FakeAccountAdapter::containing([registered(
            "other_user",
            "john@example.com",
        )]));

        let result = register_with(repo, with("john_doe", "john@example.com", "1234567"));

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
        let use_case = RegisterAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            hasher,
            Arc::new(FixedIdGeneratorAdapter(ID)),
        );

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    /// The policy runs before the hash: a rejected password never reaches the
    /// hasher, so a hasher that would fail is not called.
    #[test]
    fn rejects_the_password_before_hashing_it() {
        let hasher =
            Arc::new(FixedPasswordHashingAdapter::new().failing_on(PasswordHashingOperation::Hash));
        let use_case = RegisterAccountHandler::new(
            Arc::new(FakeAccountAdapter::empty()),
            hasher,
            Arc::new(FixedIdGeneratorAdapter(ID)),
        );

        let result = use_case.execute(with("john_doe", "john@example.com", "1234567"));

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
            Arc::new(FixedIdGeneratorAdapter(ID)),
        );

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }
}

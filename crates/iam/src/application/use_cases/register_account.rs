use crate::application::commands::register_account::RegisterAccount;
use crate::application::results::account_registered::AccountRegistered;
use crate::application::errors::application_error::ApplicationError;
use crate::application::policies::password_policy::PasswordPolicy;
use crate::application::ports::inbound::account_registration::AccountRegistrationPort;
use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::domain::aggregates::Account;
use crate::domain::value_objects::{Email, AccountId, Username};
use shared::application::common_application_error::CommonApplicationError;
use std::sync::Arc;

pub struct RegisterAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
}

impl RegisterAccountUseCase {
    pub fn new(
        account_repository: Arc<dyn AccountRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
        }
    }
}

impl AccountRegistrationPort for RegisterAccountUseCase {
    fn execute(&self, cmd: RegisterAccount) -> Result<AccountRegistered, ApplicationError> {
        let existing_email = self.account_repository.find_by_email(cmd.email.as_str());
        if existing_email.is_some() {
            return Err(ApplicationError::EmailAlreadyExists);
        }

        let existing_username = self
            .account_repository
            .find_by_username(cmd.username.as_str());
        if existing_username.is_some() {
            return Err(ApplicationError::UsernameAlreadyExists);
        }

        PasswordPolicy::validate(&cmd.password)?;

        let account_id = AccountId::generate();
        let username =
            Username::new(cmd.username).map_err(|_| ApplicationError::InvalidUsername)?;
        let email = Email::new(cmd.email).map_err(|_| ApplicationError::InvalidEmail)?;
        let hashed_password = self.password_hasher.hash(&cmd.password);

        let account = Account::register(account_id, username, email, hashed_password);

        self.account_repository
            .save(&account)
            .map_err(|_| CommonApplicationError::Infrastructure)?;

        dbg!(&account);

        Ok(AccountRegistered {
            id: account.id().as_uuid().to_string(),
            username: account.username().as_str().to_owned(),
            email: account.email().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::application::commands::register_account::RegisterAccount;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::inbound::account_registration::AccountRegistrationPort;
    use crate::application::use_cases::register_account::RegisterAccountUseCase;
    use shared::application::common_application_error::CommonApplicationError;
    use crate::application::ports::outbound::account_repository::test_utils::FakeAccountRepository;
    use crate::application::ports::outbound::password_hasher::test_utils::FakePasswordHasher;

    fn valid_input() -> RegisterAccount {
        RegisterAccount {
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        }
    }

    #[test]
    fn register_account_successfully() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_username_is_invalid() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let input = RegisterAccount {
            username: "".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        assert!(matches!(result, Err(ApplicationError::InvalidUsername)));
    }

    #[test]
    fn fails_when_email_is_invalid() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let input = RegisterAccount {
            username: "john_doe".to_string(),
            email: "invalid-email".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        assert!(matches!(result, Err(ApplicationError::InvalidEmail)));
    }

    #[test]
    fn fails_when_repository_fails() {
        let repo = Arc::new(FakeAccountRepository::fail());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(
            result,
            Err(ApplicationError::Common(
                CommonApplicationError::Infrastructure
            ))
        ));
    }

    #[test]
    fn fails_when_username_already_exists() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("john_doe"));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(ApplicationError::UsernameAlreadyExists)));
    }

    #[test]
    fn fails_when_email_already_exists() {
        let repo = Arc::new(FakeAccountRepository::with_existing_email("john@example.com"));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterAccountUseCase::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(ApplicationError::EmailAlreadyExists)));
    }
}

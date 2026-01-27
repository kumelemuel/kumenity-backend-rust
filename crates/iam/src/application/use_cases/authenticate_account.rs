use std::sync::Arc;
use shared::application::common_application_error::CommonApplicationError;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::application::commands::authenticate_account::AuthenticateAccount;
use crate::application::results::account_authenticated::AccountAuthenticated;
use crate::application::ports::inbound::account_authentication::AccountAuthenticationPort;
use crate::application::ports::outbound::token_generator::TokenGeneratorPort;
use crate::domain::aggregates::Account;

pub struct AuthenticateAccountUseCase {
    account_repository: Arc<dyn AccountRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_generator: Arc<dyn TokenGeneratorPort>,
}

impl AuthenticateAccountUseCase {
    pub fn new(
        account_repository: Arc<dyn AccountRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_generator: Arc<dyn TokenGeneratorPort>,
    ) -> Self {
        Self {
            account_repository,
            password_hasher,
            token_generator,
        }
    }
}

impl AccountAuthenticationPort for AuthenticateAccountUseCase {
    fn execute(&self, cmd: AuthenticateAccount) -> Result<AccountAuthenticated, ApplicationError> {
        let mut account: Option<Account> = None;
        let existing_username = self
            .account_repository
            .find_by_username(cmd.username.as_str());
        if existing_username.is_some() {
            account = existing_username;
        }

        if account.is_none() {
            return Err(ApplicationError::UserNotFound);
        }
        let account = account.unwrap();

        if !self.password_hasher.verify(cmd.password.as_str(), &account.password()) {
            return Err(ApplicationError::LoginFailed);
        }

        if !account.can_authenticate() {
            return Err(ApplicationError::LoginFailed);
        }

        let token = self.token_generator.generate(&account.id().as_uuid().to_string()).map_err(|_| ApplicationError::Common(CommonApplicationError::Infrastructure))?;

        Ok(AccountAuthenticated {
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
    use crate::domain::value_objects::{HashedPassword};
    use std::sync::Arc;
    use crate::application::commands::authenticate_account::AuthenticateAccount;
    use crate::application::ports::inbound::account_authentication::AccountAuthenticationPort;
    use crate::application::ports::outbound::account_repository::tests::FakeAccountRepository;
    use crate::application::ports::outbound::token_generator::TokenGeneratorPort;
    use crate::application::use_cases::authenticate_account::AuthenticateAccountUseCase;

    struct FakePasswordHasher;
    struct FakeTokenGenerator;

    impl PasswordHasherPort for FakePasswordHasher {
        fn hash(&self, _raw: &str) -> HashedPassword {
            HashedPassword::dummy()
        }

        fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool {
            password == hashed_password.as_str()
        }
    }

    impl TokenGeneratorPort for FakeTokenGenerator {
        fn generate(&self, _user_id: &str) -> Result<String, String> {
            Ok(String::from("valid_token"))
        }
    }

    // struct FakeAccountRepository {
    //     account: Option<Account>
    // }
    //
    // impl FakeAccountRepository {
    //     fn empty() -> Self {
    //         Self {
    //             account: None
    //         }
    //     }
    //
    //     fn with_user(account: Account) -> Self {
    //         Self {
    //             account: Some(account)
    //         }
    //     }
    // }
    //
    // impl AccountRepositoryPort for FakeAccountRepository {
    //     fn find_by_username(&self, _username: &str) -> Option<Account> {
    //         self.account.clone()
    //     }
    //
    //     fn find_by_email(&self, _email: &str) -> Option<Account> {
    //         self.account.clone()
    //     }
    //
    //     fn save(&self, _user: &Account) -> Result<(), String> {
    //         Ok(())
    //     }
    // }
    //
    // fn dummy_account() -> Account {
    //     Account::reconstitute(
    //         AccountId::generate(),
    //         Username::new("dummy".to_string()).unwrap(),
    //         Email::new("dummy@example.com").unwrap(),
    //         HashedPassword::dummy(),
    //         AccountStatus::Active,
    //     )
    // }
    //
    // fn dummy_deactivated_account() -> Account {
    //     Account::reconstitute(
    //         AccountId::generate(),
    //         Username::new("dummy".to_string()).unwrap(),
    //         Email::new("dummy@example.com").unwrap(),
    //         HashedPassword::dummy(),
    //         AccountStatus::Deactivated,
    //     )
    // }

    #[test]
    fn fails_when_account_not_found() {
        let repo = Arc::new(FakeAccountRepository::success());
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn fails_with_incorrect_password() {
        let repo = Arc::new(FakeAccountRepository::with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: "wrong_password".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::LoginFailed)));
    }

    #[test]
    fn fails_when_account_cannot_authenticate() {

        let repo = Arc::new(FakeAccountRepository::with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::LoginFailed)));
    }

    #[test]
    fn authenticate_account_successfully_returns_token() {

        let repo = Arc::new(FakeAccountRepository::active_with_existing_username("dummy"));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = AuthenticateAccountUseCase::new(repo, hasher, token_generator);

        let result = use_case.execute(AuthenticateAccount {
            username: "dummy".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "valid_token");
    }


}

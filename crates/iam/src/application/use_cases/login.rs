use std::sync::Arc;
use shared::application::common_application_error::CommonApplicationError;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::outbound::password_hasher_port::PasswordHasherPort;
use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
use crate::application::dto::input::login_dto::LoginDto;
use crate::application::dto::output::logged_dto::LoggedDto;
use crate::application::ports::inbound::login_port::LoginPort;
use crate::application::ports::outbound::token_generator_port::TokenGeneratorPort;
use crate::domain::aggregates::User;

pub struct Login {
    user_repository: Arc<dyn UserRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_generator: Arc<dyn TokenGeneratorPort>,
}

impl Login {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_generator: Arc<dyn TokenGeneratorPort>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
            token_generator,
        }
    }
}

impl LoginPort for Login {
    fn execute(&self, input: LoginDto) -> Result<LoggedDto, ApplicationError> {
        let mut user: Option<User> = None;
        let existing_email = self.user_repository.find_by_email(input.identify.as_str());
        if existing_email.is_none() {
            let existing_username = self
                .user_repository
                .find_by_username(input.identify.as_str());
            if existing_username.is_some() {
                user = existing_username;
            }
        } else {
            user = existing_email;
        }

        if user.is_none() {
            return Err(ApplicationError::UserNotFound);
        }
        let user = user.unwrap();

        if !self.password_hasher.verify(input.password.as_str(), &user.password()) {
            return Err(ApplicationError::LoginFailed);
        }

        if !user.can_authenticate() {
            return Err(ApplicationError::LoginFailed);
        }

        let token = self.token_generator.generate(&user.id().as_uuid().to_string()).map_err(|_| ApplicationError::Common(CommonApplicationError::Infrastructure))?;

        Ok(LoggedDto {
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::outbound::password_hasher_port::PasswordHasherPort;
    use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
    use crate::domain::aggregates::User;
    use crate::domain::value_objects::{Email, HashedPassword, UserId, UserStatus, Username};
    use std::sync::Arc;
    use crate::application::dto::input::login_dto::LoginDto;
    use crate::application::ports::inbound::login_port::LoginPort;
    use crate::application::ports::outbound::token_generator_port::TokenGeneratorPort;
    use crate::application::use_cases::login::Login;

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

    struct FakeUserRepository {
        user: Option<User>
    }

    impl FakeUserRepository {
        fn empty() -> Self {
            Self {
                user: None
            }
        }

        fn with_user(user: User) -> Self {
            Self {
                user: Some(user)
            }
        }
    }

    impl UserRepositoryPort for FakeUserRepository {
        fn find_by_username(&self, _username: &str) -> Option<User> {
            self.user.clone()
        }

        fn find_by_email(&self, _email: &str) -> Option<User> {
            self.user.clone()
        }

        fn save(&self, _user: &User) -> Result<(), String> {
            Ok(())
        }
    }

    fn dummy_user() -> User {
        User::reconstitute(
            UserId::generate(),
            Username::new("dummy".to_string()).unwrap(),
            Email::new("dummy@example.com").unwrap(),
            HashedPassword::dummy(),
            UserStatus::Active,
        )
    }

    fn dummy_inactive_user() -> User {
        User::reconstitute(
            UserId::generate(),
            Username::new("dummy".to_string()).unwrap(),
            Email::new("dummy@example.com").unwrap(),
            HashedPassword::dummy(),
            UserStatus::Deactivated,
        )
    }

    #[test]
    fn fails_when_user_not_found() {
        let repo = Arc::new(FakeUserRepository::empty());
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = Login::new(repo, hasher, token_generator);

        let result = use_case.execute(LoginDto {
            identify: "john@example.com".to_string(),
            password: "123456789".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn fails_with_incorrect_password() {
        let user = dummy_user();
        let repo = Arc::new(FakeUserRepository::with_user(user));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = Login::new(repo, hasher, token_generator);

        let result = use_case.execute(LoginDto {
            identify: "john@example.com".to_string(),
            password: "wrong_password".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::LoginFailed)));
    }

    #[test]
    fn fails_when_user_cannot_authenticate() {

        let user = dummy_inactive_user();

        let repo = Arc::new(FakeUserRepository::with_user(user));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = Login::new(repo, hasher, token_generator);

        let result = use_case.execute(LoginDto {
            identify: "john@example.com".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::LoginFailed)));
    }

    #[test]
    fn login_successfully_returns_token() {
        let user = dummy_user();

        let repo = Arc::new(FakeUserRepository::with_user(user));
        let hasher = Arc::new(FakePasswordHasher);
        let token_generator = Arc::new(FakeTokenGenerator);

        let use_case = Login::new(repo, hasher, token_generator);

        let result = use_case.execute(LoginDto {
            identify: "dummy@example.com".to_string(),
            password: HashedPassword::dummy().as_str().to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "valid_token");
    }


}

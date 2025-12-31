use crate::application::dto::input::register_user_dto::RegisterUserDto;
use crate::application::dto::output::registered_user_dto::RegisteredUserDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::policies::password_policy::PasswordPolicy;
use crate::application::ports::inbound::register_user_port::RegisterUserPort;
use crate::application::ports::outbound::password_hasher_port::PasswordHasherPort;
use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
use crate::domain::aggregates::User;
use crate::domain::value_objects::{Email, UserId, Username};
use shared::application::common_application_error::CommonApplicationError;
use std::sync::Arc;

pub struct RegisterUser {
    user_repository: Arc<dyn UserRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
}

impl RegisterUser {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }
}

impl RegisterUserPort for RegisterUser {
    fn execute(&self, input: RegisterUserDto) -> Result<RegisteredUserDto, ApplicationError> {
        let existing_email = self.user_repository.find_by_email(input.email.as_str());
        if existing_email.is_some() {
            return Err(ApplicationError::EmailAlreadyExists);
        }

        let existing_username = self
            .user_repository
            .find_by_username(input.username.as_str());
        if existing_username.is_some() {
            return Err(ApplicationError::UsernameAlreadyExists);
        }

        PasswordPolicy::validate(&input.password)?;

        let user_id = UserId::generate();
        let username =
            Username::new(input.username).map_err(|_| ApplicationError::InvalidUsername)?;
        let email = Email::new(input.email).map_err(|_| ApplicationError::InvalidEmail)?;
        let hashed_password = self.password_hasher.hash(&input.password);

        let user = User::register(user_id, username, email, hashed_password);

        self.user_repository
            .save(&user)
            .map_err(|_| CommonApplicationError::Infrastructure)?;

        Ok(RegisteredUserDto {
            id: user.id().as_uuid().to_string(),
            username: user.username().as_str().to_owned(),
            email: user.email().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::dto::input::register_user_dto::RegisterUserDto;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::inbound::register_user_port::RegisterUserPort;
    use crate::application::ports::outbound::password_hasher_port::PasswordHasherPort;
    use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
    use crate::application::use_cases::register_user::RegisterUser;
    use crate::domain::aggregates::User;
    use crate::domain::value_objects::{Email, HashedPassword, UserId, Username};
    use shared::application::common_application_error::CommonApplicationError;
    use std::sync::Arc;

    struct FakePasswordHasher;

    impl PasswordHasherPort for FakePasswordHasher {
        fn hash(&self, _raw: &str) -> HashedPassword {
            HashedPassword::dummy()
        }

        fn verify(&self, _password: &str, _hashed_password: &HashedPassword) -> bool {
            todo!()
        }
    }

    struct FakeUserRepository {
        should_fail: bool,
        existing_username: Option<String>,
        existing_email: Option<String>,
    }

    impl FakeUserRepository {
        fn success() -> Self {
            Self {
                should_fail: false,
                existing_username: None,
                existing_email: None,
            }
        }

        fn fail() -> Self {
            Self {
                should_fail: true,
                existing_username: None,
                existing_email: None,
            }
        }

        fn with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                existing_username: Some(username.to_string()),
                existing_email: None,
            }
        }

        fn with_existing_email(email: &str) -> Self {
            Self {
                should_fail: false,
                existing_username: None,
                existing_email: Some(email.to_string()),
            }
        }
    }

    impl UserRepositoryPort for FakeUserRepository {
        fn find_by_username(&self, username: &str) -> Option<User> {
            self.existing_username
                .as_ref()
                .filter(|u| u.as_str() == username)
                .map(|_| dummy_user())
        }

        fn find_by_email(&self, email: &str) -> Option<User> {
            self.existing_email
                .as_ref()
                .filter(|e| e.as_str() == email)
                .map(|_| dummy_user())
        }

        fn save(&self, _user: &User) -> Result<(), String> {
            if self.should_fail {
                Err("Unexpected error".to_string())
            } else {
                Ok(())
            }
        }
    }

    fn dummy_user() -> User {
        User::register(
            UserId::generate(),
            Username::new("dummy".to_string()).unwrap(),
            Email::new("dummy@example.com").unwrap(),
            HashedPassword::dummy(),
        )
    }

    fn valid_input() -> RegisterUserDto {
        RegisterUserDto {
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        }
    }

    #[test]
    fn registers_user_successfully() {
        let repo = Arc::new(FakeUserRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_username_is_invalid() {
        let repo = Arc::new(FakeUserRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

        let input = RegisterUserDto {
            username: "".to_string(),
            email: "john@example.com".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        assert!(matches!(result, Err(ApplicationError::InvalidUsername)));
    }

    #[test]
    fn fails_when_email_is_invalid() {
        let repo = Arc::new(FakeUserRepository::success());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

        let input = RegisterUserDto {
            username: "john_doe".to_string(),
            email: "invalid-email".to_string(),
            password: "password123456789".to_string(),
        };

        let result = use_case.execute(input);

        assert!(matches!(result, Err(ApplicationError::InvalidEmail)));
    }

    #[test]
    fn fails_when_repository_fails() {
        let repo = Arc::new(FakeUserRepository::fail());
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

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
        let repo = Arc::new(FakeUserRepository::with_existing_username("john_doe"));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(
            result,
            Err(ApplicationError::UsernameAlreadyExists)
        ));
    }

    #[test]
    fn fails_when_email_already_exists() {
        let repo = Arc::new(FakeUserRepository::with_existing_email("john@example.com"));
        let hasher = Arc::new(FakePasswordHasher);

        let use_case = RegisterUser::new(repo, hasher);

        let result = use_case.execute(valid_input());

        assert!(matches!(result, Err(ApplicationError::EmailAlreadyExists)));
    }
}

use std::sync::Arc;
use shared::application::common_application_error::CommonApplicationError;
use crate::application::dto::input::validate_user_dto::ValidateUserDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::inbound::validate_user_port::ValidateUserPort;
use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
use crate::domain::value_objects::{CodeValidation, UserStatus};

pub struct ValidateUser {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl ValidateUser {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repository }
    }
}

impl ValidateUserPort for ValidateUser {
    fn execute(&self, data: ValidateUserDto) -> Result<bool, ApplicationError> {
        let user = self.user_repository.find_by_email(data.email.as_str());
        if user.is_none() {
            return Err(ApplicationError::UserNotFound);
        }
        let mut user = user.unwrap();
        let code_validation = CodeValidation::new(data.code).map_err(|_| ApplicationError::InvalidCodeValidation)?;
        if user.status().ne(&UserStatus::Registered { code_validation }) {
            return Err(ApplicationError::ActivationFailed);
        }

        user.activate().expect("error while activating user");
        self.user_repository
            .save(&user)
            .map_err(|_| CommonApplicationError::Infrastructure)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::application::dto::input::validate_user_dto::ValidateUserDto;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::inbound::validate_user_port::ValidateUserPort;
    use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
    use crate::application::use_cases::validate_user::ValidateUser;
    use crate::domain::aggregates::User;
    use crate::domain::value_objects::{CodeValidation, Email, HashedPassword, UserId, UserStatus, Username};

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
        User::reconstitute(
            UserId::generate(),
            Username::new("dummy".to_string()).unwrap(),
            Email::new("dummy@example.com").unwrap(),
            HashedPassword::dummy(),
            UserStatus::Registered { code_validation: CodeValidation::new(123123).unwrap() },
        )
    }

    #[test]
    fn validate_user_successfully() {
        let repo = Arc::new(FakeUserRepository::with_existing_email("john@example.com"));

        let use_case = ValidateUser::new(repo);

        let result = use_case.execute(ValidateUserDto {
            email: "john@example.com".to_string(),
            code: 123123
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_email_not_found() {
        let repo = Arc::new(FakeUserRepository::success());

        let use_case = ValidateUser::new(repo);

        let result = use_case.execute(ValidateUserDto {
            email: "john@example.com".to_string(),
            code: 123123
        });

        assert!(matches!(result, Err(ApplicationError::UserNotFound)));
    }

    #[test]
    fn fails_when_code_not_match() {
        let repo = Arc::new(FakeUserRepository::with_existing_email("john@example.com"));

        let use_case = ValidateUser::new(repo);

        let result = use_case.execute(ValidateUserDto {
            email: "john@example.com".to_string(),
            code: 123111
        });

        assert!(matches!(result, Err(ApplicationError::ActivationFailed)));
    }
}
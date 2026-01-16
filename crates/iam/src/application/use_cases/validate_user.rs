use std::sync::Arc;
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
        Ok(true)
    }
}
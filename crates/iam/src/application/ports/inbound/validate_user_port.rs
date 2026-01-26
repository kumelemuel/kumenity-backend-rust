use crate::application::dto::input::validate_user_dto::ValidateUserDto;
use crate::application::errors::application_error::ApplicationError;

pub trait ValidateUserPort {
    fn execute(&self, data: ValidateUserDto) -> Result<bool, ApplicationError>;
}

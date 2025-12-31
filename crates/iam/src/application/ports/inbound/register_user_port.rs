use crate::application::dto::input::register_user_dto::RegisterUserDto;
use crate::application::dto::output::registered_user_dto::RegisteredUserDto;
use crate::application::errors::application_error::ApplicationError;

pub trait RegisterUserPort {
    fn execute(&self, data: RegisterUserDto) -> Result<RegisteredUserDto, ApplicationError>;
}

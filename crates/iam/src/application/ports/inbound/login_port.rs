use crate::application::dto::input::login_dto::LoginDto;
use crate::application::dto::output::logged_dto::LoggedDto;
use crate::application::errors::application_error::ApplicationError;

pub trait LoginPort {
    fn execute(&self, data: LoginDto) -> Result<LoggedDto, ApplicationError>;
}

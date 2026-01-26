use shared::application::common_application_error::CommonApplicationError;

#[derive(Debug)]
pub enum ApplicationError {
    Common(CommonApplicationError),
    UserNotFound,
    LoginFailed,
    ActivationFailed,
    UsernameAlreadyExists,
    EmailAlreadyExists,
    InvalidEmail,
    InvalidUsername,
    InvalidPassword,
    InvalidCodeValidation
}

impl From<CommonApplicationError> for ApplicationError {
    fn from(err: CommonApplicationError) -> Self {
        ApplicationError::Common(err)
    }
}

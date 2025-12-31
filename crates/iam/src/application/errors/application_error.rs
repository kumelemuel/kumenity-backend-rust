use shared::application::common_application_error::CommonApplicationError;

pub enum ApplicationError {
    Common(CommonApplicationError),
    UsernameAlreadyExists,
    EmailAlreadyExists,
    InvalidEmail,
    InvalidUsername,
    InvalidPassword,
}

impl From<CommonApplicationError> for ApplicationError {
    fn from(err: CommonApplicationError) -> Self {
        ApplicationError::Common(err)
    }
}

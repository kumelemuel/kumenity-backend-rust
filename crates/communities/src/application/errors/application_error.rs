use shared::application::common_application_error::CommonApplicationError;

#[derive(Debug)]
pub enum ApplicationError {
    Common(CommonApplicationError),
    InvalidName,
    InvalidSlug,
}

impl From<CommonApplicationError> for ApplicationError {
    fn from(err: CommonApplicationError) -> Self {
        ApplicationError::Common(err)
    }
}

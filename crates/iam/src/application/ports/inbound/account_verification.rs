use crate::application::commands::verify_account::VerifyAccount;
use crate::application::errors::application_error::ApplicationError;

pub trait AccountVerificationPort {
    fn execute(&self, data: VerifyAccount) -> Result<bool, ApplicationError>;
}

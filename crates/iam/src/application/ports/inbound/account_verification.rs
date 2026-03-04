use crate::application::commands::verify_account::VerifyAccount;
use shared::error::SystemError;

pub trait AccountVerificationPort {
    fn execute(&self, data: VerifyAccount) -> Result<bool, SystemError>;
}

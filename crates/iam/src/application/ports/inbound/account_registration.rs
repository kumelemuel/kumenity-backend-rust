use crate::application::commands::register_account::RegisterAccount;
use crate::application::results::account_registered::AccountRegistered;
use crate::application::errors::application_error::ApplicationError;

pub trait AccountRegistrationPort {
    fn execute(&self, data: RegisterAccount) -> Result<AccountRegistered, ApplicationError>;
}

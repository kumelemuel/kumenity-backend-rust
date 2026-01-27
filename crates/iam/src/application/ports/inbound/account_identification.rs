use crate::application::commands::identify_account::IdentifyAccount;
use crate::application::errors::application_error::ApplicationError;
use crate::application::results::account_identified::AccountIdentified;

pub trait AccountIdentificationPort {
    fn execute(&self, data: IdentifyAccount) -> Result<AccountIdentified, ApplicationError>;
}

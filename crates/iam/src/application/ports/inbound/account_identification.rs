use crate::application::{
    commands::identify_account::IdentifyAccount, results::account_identified::AccountIdentified,
};
use shared::error::SystemError;

pub trait AccountIdentificationPort {
    fn execute(&self, data: IdentifyAccount) -> Result<AccountIdentified, SystemError>;
}

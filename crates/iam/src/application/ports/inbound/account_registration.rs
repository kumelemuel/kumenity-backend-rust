use crate::application::{
    commands::register_account::RegisterAccount, results::account_registered::AccountRegistered,
};
use shared::error::SystemError;

pub trait AccountRegistrationPort {
    fn execute(&self, data: RegisterAccount) -> Result<AccountRegistered, SystemError>;
}

use crate::application::{
    commands::authenticate_account::AuthenticateAccount,
    results::account_authenticated::AccountAuthenticated,
};
use shared::error::SystemError;

pub trait AccountAuthenticationPort {
    fn execute(&self, data: AuthenticateAccount) -> Result<AccountAuthenticated, SystemError>;
}

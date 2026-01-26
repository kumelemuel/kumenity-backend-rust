use crate::application::commands::authenticate_account::AuthenticateAccount;
use crate::application::results::account_authenticated::AccountAuthenticated;
use crate::application::errors::application_error::ApplicationError;

pub trait AccountAuthenticationPort {
    fn execute(&self, data: AuthenticateAccount) -> Result<AccountAuthenticated, ApplicationError>;
}

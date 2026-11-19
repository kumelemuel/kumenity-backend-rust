use crate::application::commands::authenticate_account::{
    command::AuthenticateAccountCommand, response::AuthenticateAccountResponse,
};
use shared::error::SystemError;

pub trait AuthenticateAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, SystemError>;
}

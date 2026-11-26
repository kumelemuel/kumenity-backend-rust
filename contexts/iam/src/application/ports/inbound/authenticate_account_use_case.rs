use crate::application::commands::authenticate_account::{
    AuthenticateAccountCommand, AuthenticateAccountResponse,
};
use shared::error::SystemError;

pub trait AuthenticateAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, SystemError>;
}

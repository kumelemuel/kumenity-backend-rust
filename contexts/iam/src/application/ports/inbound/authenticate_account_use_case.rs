use crate::application::commands::authenticate_account::{
    AuthenticateAccountCommand, AuthenticateAccountError, AuthenticateAccountResponse,
};
use shared::Failure;

pub trait AuthenticateAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: AuthenticateAccountCommand,
    ) -> Result<AuthenticateAccountResponse, Failure<AuthenticateAccountError>>;
}

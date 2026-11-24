use crate::application::commands::register_account::{
    RegisterAccountCommand, RegisterAccountError, RegisterAccountResponse,
};
use shared::Failure;

pub trait RegisterAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: RegisterAccountCommand,
    ) -> Result<RegisterAccountResponse, Failure<RegisterAccountError>>;
}

use crate::application::commands::register_account::{
    command::RegisterAccountCommand, response::RegisterAccountResponse,
};
use shared::error::SystemError;

pub trait RegisterAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: RegisterAccountCommand,
    ) -> Result<RegisterAccountResponse, SystemError>;
}

use crate::application::commands::verify_account::command::VerifyAccountCommand;
use shared::error::SystemError;

pub trait VerifyAccountUseCase: Send + Sync {
    fn execute(&self, command: VerifyAccountCommand) -> Result<(), SystemError>;
}

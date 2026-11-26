use crate::application::commands::verify_account::{
    VerifyAccountCommand, VerifyAccountError, VerifyAccountResponse,
};
use shared::Failure;

pub trait VerifyAccountUseCase: Send + Sync {
    fn execute(
        &self,
        command: VerifyAccountCommand,
    ) -> Result<VerifyAccountResponse, Failure<VerifyAccountError>>;
}

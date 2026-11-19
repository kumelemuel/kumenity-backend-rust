use crate::application::commands::verify_account::command::VerifyAccountCommand;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub validation_code: u32,
}

impl From<VerifyRequest> for VerifyAccountCommand {
    fn from(req: VerifyRequest) -> Self {
        VerifyAccountCommand {
            email: req.email,
            code: req.validation_code,
        }
    }
}


use serde::Deserialize;
use iam::application::commands::verify_account::VerifyAccount;

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub validation_code: u32,
}

impl From<VerifyRequest> for VerifyAccount {
    fn from(req: VerifyRequest) -> Self {
        VerifyAccount {
            email: req.email,
            code: req.validation_code,
        }
    }
}

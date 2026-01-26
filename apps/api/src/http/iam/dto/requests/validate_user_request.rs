
use serde::Deserialize;
use iam::application::commands::verify_account::VerifyAccount;

#[derive(Debug, Deserialize)]
pub struct ValidateUserRequest {
    pub email: String,
    pub validation_code: u32,
}

impl From<ValidateUserRequest> for VerifyAccount {
    fn from(req: ValidateUserRequest) -> Self {
        VerifyAccount {
            email: req.email,
            code: req.validation_code,
        }
    }
}

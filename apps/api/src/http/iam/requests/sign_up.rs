use iam::application::commands::register_account::RegisterAccount;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<SignUpRequest> for RegisterAccount {
    fn from(req: SignUpRequest) -> Self {
        RegisterAccount {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

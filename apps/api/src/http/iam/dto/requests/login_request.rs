
use serde::Deserialize;
use iam::application::commands::authenticate_account::AuthenticateAccount;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub identify: String,
    pub password: String,
}

impl From<LoginRequest> for AuthenticateAccount {
    fn from(req: LoginRequest) -> Self {
        AuthenticateAccount {
            identify: req.identify,
            password: req.password,
        }
    }
}

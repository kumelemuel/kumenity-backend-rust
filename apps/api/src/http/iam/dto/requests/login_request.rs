
use serde::Deserialize;
use iam::application::commands::authenticate_account::AuthenticateAccount;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl From<LoginRequest> for AuthenticateAccount {
    fn from(req: LoginRequest) -> Self {
        AuthenticateAccount {
            username: req.username,
            password: req.password,
        }
    }
}

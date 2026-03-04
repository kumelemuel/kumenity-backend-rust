use iam::application::commands::authenticate_account::AuthenticateAccount;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

impl From<SignInRequest> for AuthenticateAccount {
    fn from(req: SignInRequest) -> Self {
        AuthenticateAccount {
            username: req.username,
            password: req.password,
        }
    }
}

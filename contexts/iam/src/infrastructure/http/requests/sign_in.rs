use crate::application::commands::authenticate_account::command::AuthenticateAccountCommand;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

impl From<SignInRequest> for AuthenticateAccountCommand {
    fn from(req: SignInRequest) -> Self {
        AuthenticateAccountCommand {
            username: req.username,
            password: req.password,
        }
    }
}

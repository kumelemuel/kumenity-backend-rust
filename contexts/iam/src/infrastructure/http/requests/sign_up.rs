use crate::application::commands::register_account::command::RegisterAccountCommand;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<SignUpRequest> for RegisterAccountCommand {
    fn from(req: SignUpRequest) -> Self {
        RegisterAccountCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

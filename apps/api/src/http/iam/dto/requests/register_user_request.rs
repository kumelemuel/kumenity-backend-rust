use iam::application::commands::register_account::RegisterAccount;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterUserRequest> for RegisterAccount {
    fn from(req: RegisterUserRequest) -> Self {
        RegisterAccount {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

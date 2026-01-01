use iam::application::dto::input::register_user_dto::RegisterUserDto;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterUserRequest> for RegisterUserDto {
    fn from(req: RegisterUserRequest) -> Self {
        RegisterUserDto {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

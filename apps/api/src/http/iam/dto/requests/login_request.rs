
use serde::Deserialize;
use iam::application::dto::input::login_dto::LoginDto;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub identify: String,
    pub password: String,
}

impl From<LoginRequest> for LoginDto {
    fn from(req: LoginRequest) -> Self {
        LoginDto {
            identify: req.identify,
            password: req.password,
        }
    }
}

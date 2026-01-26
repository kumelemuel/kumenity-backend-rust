
use serde::Deserialize;
use iam::application::dto::input::validate_user_dto::ValidateUserDto;

#[derive(Debug, Deserialize)]
pub struct ValidateUserRequest {
    pub email: String,
    pub validation_code: u32,
}

impl From<ValidateUserRequest> for ValidateUserDto {
    fn from(req: ValidateUserRequest) -> Self {
        ValidateUserDto {
            email: req.email,
            code: req.validation_code,
        }
    }
}

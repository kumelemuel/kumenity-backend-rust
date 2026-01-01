use iam::application::dto::output::registered_user_dto::RegisteredUserDto;
use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub uuid: String,
    pub username: String,
    pub email: String,
}

impl From<RegisteredUserDto> for RegisterUserResponse {
    fn from(dto: RegisteredUserDto) -> Self {
        Self {
            uuid: dto.id,
            username: dto.username,
            email: dto.email,
        }
    }
}

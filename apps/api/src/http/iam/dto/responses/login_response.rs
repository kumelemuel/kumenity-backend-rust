use serde::Serialize;
use iam::application::dto::output::logged_dto::LoggedDto;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

impl From<LoggedDto> for LoginResponse {
    fn from(dto: LoggedDto) -> Self {
        Self {
            token: dto.token,
        }
    }
}

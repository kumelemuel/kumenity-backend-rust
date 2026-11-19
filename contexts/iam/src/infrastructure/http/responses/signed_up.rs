use crate::application::commands::register_account::response::RegisterAccountResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignedUpResponse {
    pub uuid: String,
    pub username: String,
    pub email: String,
}

impl From<RegisterAccountResponse> for SignedUpResponse {
    fn from(dto: RegisterAccountResponse) -> Self {
        Self {
            uuid: dto.id,
            username: dto.username,
            email: dto.email,
        }
    }
}

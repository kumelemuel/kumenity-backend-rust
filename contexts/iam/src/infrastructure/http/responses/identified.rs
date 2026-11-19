use crate::application::queries::identify_account::response::IdentifyAccountResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct IdentifiedResponse {
    pub username: String,
    pub status: String,
}

impl From<IdentifyAccountResponse> for IdentifiedResponse {
    fn from(dto: IdentifyAccountResponse) -> Self {
        Self {
            username: dto.username,
            status: dto.status,
        }
    }
}

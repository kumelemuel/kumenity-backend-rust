use iam::application::results::account_identified::AccountIdentified;
use serde::Serialize;

#[derive(Serialize)]
pub struct IdentifiedResponse {
    pub username: String,
    pub status: String,
}

impl From<AccountIdentified> for IdentifiedResponse {
    fn from(dto: AccountIdentified) -> Self {
        Self {
            username: dto.username,
            status: dto.status,
        }
    }
}

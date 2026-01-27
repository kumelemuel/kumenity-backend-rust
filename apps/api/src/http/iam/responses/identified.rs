use serde::Serialize;
use iam::application::results::account_identified::AccountIdentified;

#[derive(Serialize)]
pub struct IdentifiedResponse {
    pub username: String,
}

impl From<AccountIdentified> for IdentifiedResponse {
    fn from(dto: AccountIdentified) -> Self {
        Self {
            username: dto.username,
        }
    }
}

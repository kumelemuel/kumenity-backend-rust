use iam::application::results::account_registered::AccountRegistered;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignedUpResponse {
    pub uuid: String,
    pub username: String,
    pub email: String,
}

impl From<AccountRegistered> for SignedUpResponse {
    fn from(dto: AccountRegistered) -> Self {
        Self {
            uuid: dto.id,
            username: dto.username,
            email: dto.email,
        }
    }
}

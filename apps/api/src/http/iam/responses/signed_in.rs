use serde::Serialize;
use iam::application::results::account_authenticated::AccountAuthenticated;

#[derive(Serialize)]
pub struct SignedInResponse {
    pub token: String,
}

impl From<AccountAuthenticated> for SignedInResponse {
    fn from(dto: AccountAuthenticated) -> Self {
        Self {
            token: dto.token,
        }
    }
}

use serde::Serialize;
use iam::application::results::account_authenticated::AccountAuthenticated;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

impl From<AccountAuthenticated> for LoginResponse {
    fn from(dto: AccountAuthenticated) -> Self {
        Self {
            token: dto.token,
        }
    }
}

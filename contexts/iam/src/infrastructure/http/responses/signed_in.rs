use crate::application::commands::authenticate_account::response::AuthenticateAccountResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignedInResponse {
    pub token: String,
}

impl From<AuthenticateAccountResponse> for SignedInResponse {
    fn from(dto: AuthenticateAccountResponse) -> Self {
        Self { token: dto.token }
    }
}

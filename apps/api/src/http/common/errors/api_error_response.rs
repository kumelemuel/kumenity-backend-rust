use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}

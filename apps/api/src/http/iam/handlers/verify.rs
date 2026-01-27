use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::verify_account::VerifyAccount;
use crate::http::iam::requests::verify::VerifyRequest;
use crate::http::iam::errors::error_mapper::map_application_error;

pub async fn verify_handler(
    State(state): State<AppState>,
    Json(request): Json<VerifyRequest>,
) -> Response {
    match state.verify_account.execute(VerifyAccount::from(request)) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => map_application_error(err),
    }
}

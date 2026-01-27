use crate::http::iam::requests::sign_up::SignUpRequest;
use crate::http::iam::responses::signed_up::SignedUpResponse;
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::register_account::RegisterAccount;
use crate::http::iam::errors::error_mapper::map_application_error;

pub async fn sign_up_handler(
    State(state): State<AppState>,
    Json(request): Json<SignUpRequest>,
) -> Response {
    match state.register_account.execute(RegisterAccount::from(request)) {
        Ok(result) => (StatusCode::CREATED, Json(SignedUpResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}
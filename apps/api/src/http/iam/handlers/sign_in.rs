use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::authenticate_account::AuthenticateAccount;
use crate::http::iam::requests::sign_in::SignInRequest;
use crate::http::iam::responses::signed_in::SignedInResponse;
use crate::http::iam::errors::error_mapper::map_application_error;

pub async fn sign_in_handler(
    State(state): State<AppState>,
    Json(request): Json<SignInRequest>,
) -> Response {
    match state.authenticate_account.execute(AuthenticateAccount::from(request)) {
        Ok(result) => (StatusCode::OK, Json(SignedInResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

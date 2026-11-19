use crate::{
    http::iam::{
        errors::error_mapper::map_application_error, requests::sign_in::SignInRequest,
        responses::signed_in::SignedInResponse,
    },
    state::app::AppState,
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::authenticate_account::AuthenticateAccount;

pub async fn sign_in_handler(
    State(state): State<AppState>,
    Json(request): Json<SignInRequest>,
) -> Response {
    match state
        .iam
        .authenticate_account
        .execute(AuthenticateAccount::from(request))
    {
        Ok(result) => (StatusCode::OK, Json(SignedInResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

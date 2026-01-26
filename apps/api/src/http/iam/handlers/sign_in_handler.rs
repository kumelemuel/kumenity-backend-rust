use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::authenticate_account::AuthenticateAccount;
use crate::http::iam::dto::requests::login_request::LoginRequest;
use crate::http::iam::dto::responses::login_response::LoginResponse;
use crate::http::iam::errors::error_mapper::map_application_error;

pub async fn sign_in_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Response {
    match state.login.execute(AuthenticateAccount::from(request)) {
        Ok(logged) => (StatusCode::OK, Json(LoginResponse::from(logged))).into_response(),
        Err(err) => map_application_error(err),
    }
}

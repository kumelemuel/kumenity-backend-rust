use crate::http::iam::dto::requests::register_user_request::RegisterUserRequest;
use crate::http::iam::dto::responses::register_user_response::RegisterUserResponse;
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
    Json(request): Json<RegisterUserRequest>,
) -> Response {
    match state.register_user.execute(RegisterAccount::from(request)) {
        Ok(user) => (StatusCode::CREATED, Json(RegisterUserResponse::from(user))).into_response(),
        Err(err) => map_application_error(err),
    }
}
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::dto::input::validate_user_dto::ValidateUserDto;
use crate::http::iam::dto::requests::validate_user_request::ValidateUserRequest;
use crate::http::iam::errors::error_mapper::map_application_error;

pub async fn validate_user_handler(
    State(state): State<AppState>,
    Json(request): Json<ValidateUserRequest>,
) -> Response {
    match state.validate_user.execute(ValidateUserDto::from(request)) {
        Ok(logged) => (StatusCode::OK, Json(logged)).into_response(),
        Err(err) => map_application_error(err),
    }
}

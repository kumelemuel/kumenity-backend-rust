use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    response::{Response},
};
use axum::response::IntoResponse;
use http::{HeaderMap, StatusCode};
use iam::application::errors::application_error::ApplicationError;
use shared::application::common_application_error::CommonApplicationError;
use crate::http::communities::requests::create::CreateRequest;
use crate::http::iam::errors::error_mapper::map_application_error;
use crate::middleware::auth::authenticate;

pub async fn create_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(_): Json<CreateRequest>,
) -> Response {
    let _ = match authenticate(&headers, state.token_validator) {
        Ok(auth) => auth,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    map_application_error(ApplicationError::Common(CommonApplicationError::Infrastructure))
    
}

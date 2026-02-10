use axum::{
    Json,
    extract::State,
    response::{Response},
};
use axum::response::IntoResponse;
use http::{HeaderMap, StatusCode};
use communities::application::commands::create_community::CreateCommunity;
use crate::http::communities::errors::error_mapper::map_application_error;
use crate::http::communities::requests::create::CreateRequest;
use crate::http::communities::responses::created::CreatedResponse;
use crate::middleware::auth::authenticate;
use crate::state::app::AppState;

pub async fn create_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRequest>,
) -> Response {
    let auth_context = match authenticate(&headers, state.token_validator) {
        Ok(auth) => auth,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match state.communities.create_community.execute(CreateCommunity::from(request), auth_context) {
        Ok(result) => (StatusCode::OK, Json(CreatedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
    
}

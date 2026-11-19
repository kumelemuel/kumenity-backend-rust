use crate::{
    http::communities::{
        errors::error_mapper::map_application_error, requests::create::CreateRequest,
        responses::created::CreatedResponse,
    },
    middleware::auth::authenticate,
    state::app::AppState,
};
use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use communities::application::commands::create_community::CreateCommunity;
use http::{HeaderMap, StatusCode};

pub async fn create_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRequest>,
) -> Response {
    let auth_context = match authenticate(&headers, state.token_validator) {
        Ok(auth) => auth,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match state
        .communities
        .create_community
        .execute(CreateCommunity::from(request), auth_context)
    {
        Ok(result) => (StatusCode::OK, Json(CreatedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

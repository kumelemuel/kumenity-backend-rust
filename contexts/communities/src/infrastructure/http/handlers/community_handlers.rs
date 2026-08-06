use crate::{
    application::{
        commands::create_community::command::CreateCommunityCommand,
        ports::inbound::create_community_use_case::CreateCommunityUseCase,
    },
    infrastructure::http::{
        errors::error_mapper::map_application_error, requests::create::CreateRequest,
        responses::created::CreatedResponse,
    },
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use http::StatusCode;
use shared::domain::model::verified_identity::VerifiedIdentity;
use std::sync::Arc;

pub async fn create(
    State(use_case): State<Arc<dyn CreateCommunityUseCase>>,
    Extension(identity): Extension<VerifiedIdentity>,
    Json(request): Json<CreateRequest>,
) -> impl IntoResponse {
    match use_case.execute(CreateCommunityCommand::from(request), identity) {
        Ok(result) => (StatusCode::OK, Json(CreatedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

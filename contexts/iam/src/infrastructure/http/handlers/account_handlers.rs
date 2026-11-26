use crate::{
    application::{
        ports::inbound::identify_account_use_case::IdentifyAccountUseCase,
        queries::identify_account::IdentifyAccountQuery,
    },
    infrastructure::http::{
        errors::error_mapper::map_application_error, requests::identify::IdentifyRequest,
        responses::identified::IdentifiedResponse,
    },
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn identify(
    State(use_case): State<Arc<dyn IdentifyAccountUseCase>>,
    Json(request): Json<IdentifyRequest>,
) -> impl IntoResponse {
    match use_case.execute(IdentifyAccountQuery::from(request)) {
        Ok(result) => (StatusCode::OK, Json(IdentifiedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::commands::identify_account::IdentifyAccount;
use crate::http::iam::errors::error_mapper::map_application_error;
use crate::http::iam::requests::identify::IdentifyRequest;
use crate::http::iam::responses::identified::IdentifiedResponse;
use crate::state::app::AppState;

pub async fn identify_handler(
    State(state): State<AppState>,
    Json(request): Json<IdentifyRequest>,
) -> Response {
    match state.iam.identify_account.execute(IdentifyAccount::from(request)) {
        Ok(result) => (StatusCode::OK, Json(IdentifiedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

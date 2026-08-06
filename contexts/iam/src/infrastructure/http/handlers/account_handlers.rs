use crate::{
    application::{
        commands::{
            authenticate_account::command::AuthenticateAccountCommand,
            register_account::command::RegisterAccountCommand,
            verify_account::command::VerifyAccountCommand,
        },
        ports::inbound::{
            authenticate_account_use_case::AuthenticateAccountUseCase,
            identify_account_use_case::IdentifyAccountUseCase,
            register_account_use_case::RegisterAccountUseCase,
            verify_account_use_case::VerifyAccountUseCase,
        },
        queries::identify_account::query::IdentifyAccountQuery,
    },
    infrastructure::http::{
        errors::error_mapper::map_application_error,
        requests::{
            identify::IdentifyRequest, sign_in::SignInRequest, sign_up::SignUpRequest,
            verify::VerifyRequest,
        },
        responses::{
            identified::IdentifiedResponse, signed_in::SignedInResponse,
            signed_up::SignedUpResponse,
        },
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn register(
    State(use_case): State<Arc<dyn RegisterAccountUseCase>>,
    Json(request): Json<SignUpRequest>,
) -> impl IntoResponse {
    match use_case.execute(RegisterAccountCommand::from(request)) {
        Ok(result) => (StatusCode::CREATED, Json(SignedUpResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

pub async fn authenticate(
    State(use_case): State<Arc<dyn AuthenticateAccountUseCase>>,
    Json(request): Json<SignInRequest>,
) -> impl IntoResponse {
    match use_case.execute(AuthenticateAccountCommand::from(request)) {
        Ok(result) => (StatusCode::OK, Json(SignedInResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

pub async fn verify(
    State(use_case): State<Arc<dyn VerifyAccountUseCase>>,
    Json(request): Json<VerifyRequest>,
) -> impl IntoResponse {
    match use_case.execute(VerifyAccountCommand::from(request)) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => map_application_error(err),
    }
}

pub async fn identify(
    State(use_case): State<Arc<dyn IdentifyAccountUseCase>>,
    Json(request): Json<IdentifyRequest>,
) -> impl IntoResponse {
    match use_case.execute(IdentifyAccountQuery::from(request)) {
        Ok(result) => (StatusCode::OK, Json(IdentifiedResponse::from(result))).into_response(),
        Err(err) => map_application_error(err),
    }
}

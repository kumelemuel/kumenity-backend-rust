use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use communities::application::errors::application_error::ApplicationError;
use shared::application::common_application_error::CommonApplicationError;
use crate::http::common::errors::api_error_response::ApiErrorResponse;

pub fn map_application_error(error: ApplicationError) -> Response {
    match error {
        ApplicationError::InvalidSlug => {
            let body = ApiErrorResponse {
                code: "INVALID_SLUG".to_string(),
                message: "Slug is invalid".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidName => {
            let body = ApiErrorResponse {
                code: "INVALID_NAME".to_string(),
                message: "Community name is invalid".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::SlugAlreadyExists => {
            let body = ApiErrorResponse {
                code: "SLUG_ALREADY_EXISTS".to_string(),
                message: "Slug is already in use".to_string(),
            };
            (StatusCode::CONFLICT, Json(body)).into_response()
        }
        ApplicationError::Common(CommonApplicationError::Infrastructure) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        ApplicationError::Common(CommonApplicationError::Unauthorized) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use iam::application::errors::application_error::ApplicationError;
use shared::application::common_application_error::CommonApplicationError;
use crate::http::common::errors::api_error_response::ApiErrorResponse;

pub fn map_application_error(error: ApplicationError) -> Response {
    match error {
        ApplicationError::UsernameAlreadyExists => {
            let body = ApiErrorResponse {
                code: "USERNAME_ALREADY_EXISTS".to_string(),
                message: "Username is already in use".to_string(),
            };
            (StatusCode::CONFLICT, Json(body)).into_response()
        }
        ApplicationError::EmailAlreadyExists => {
            let body = ApiErrorResponse {
                code: "EMAIL_ALREADY_EXISTS".to_string(),
                message: "E-mail is already in use".to_string(),
            };
            (StatusCode::CONFLICT, Json(body)).into_response()
        }
        ApplicationError::InvalidUsername => {
            let body = ApiErrorResponse {
                code: "INVALID_USERNAME".to_string(),
                message: "Invalid username".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidEmail => {
            let body = ApiErrorResponse {
                code: "INVALID_EMAIL".to_string(),
                message: "Invalid e-mail".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidPassword => {
            let body = ApiErrorResponse {
                code: "INVALID_PASSWORD".to_string(),
                message: "Invalid password".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidCodeValidation => {
            let body = ApiErrorResponse {
                code: "INVALID_CODE_VALIDATION".to_string(),
                message: "Invalid code validation".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::UserNotFound => {
            let body = ApiErrorResponse {
                code: "USER_NOT_FOUND".to_string(),
                message: "User not found".to_string(),
            };
            (StatusCode::NOT_FOUND, Json(body)).into_response()
        }
        ApplicationError::LoginFailed => {
            let body = ApiErrorResponse {
                code: "LOGIN_FAILED".to_string(),
                message: "Login failed".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(body)).into_response()
        }
        ApplicationError::ActivationFailed => {
            let body = ApiErrorResponse {
                code: "ACTIVATION_FAILED".to_string(),
                message: "Activation failed".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(body)).into_response()
        }
        ApplicationError::Common(CommonApplicationError::Infrastructure) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        ApplicationError::Common(CommonApplicationError::Unauthorized) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use shared::error::SystemError;

use crate::http::common::errors::api_error_response::ApiErrorResponse;

pub fn map_application_error(error: SystemError) -> Response {
    match error {
        _ => {
            let body = ApiErrorResponse {
                code: "GENERIC".to_string(),
                message: "GENERIC".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        } // SystemError::UsernameAlreadyExists => {
          //     let body = ApiErrorResponse {
          //         code: "USERNAME_ALREADY_EXISTS".to_string(),
          //         message: "Username is already in use".to_string(),
          //     };
          //     (StatusCode::CONFLICT, Json(body)).into_response()
          // }
          // SystemError::EmailAlreadyExists => {
          //     let body = ApiErrorResponse {
          //         code: "EMAIL_ALREADY_EXISTS".to_string(),
          //         message: "E-mail is already in use".to_string(),
          //     };
          //     (StatusCode::CONFLICT, Json(body)).into_response()
          // }
          // SystemError::InvalidUsername => {
          //     let body = ApiErrorResponse {
          //         code: "INVALID_USERNAME".to_string(),
          //         message: "Invalid username".to_string(),
          //     };
          //     (StatusCode::BAD_REQUEST, Json(body)).into_response()
          // }
          // SystemError::InvalidEmail => {
          //     let body = ApiErrorResponse {
          //         code: "INVALID_EMAIL".to_string(),
          //         message: "Invalid e-mail".to_string(),
          //     };
          //     (StatusCode::BAD_REQUEST, Json(body)).into_response()
          // }
          // SystemError::InvalidPassword => {
          //     let body = ApiErrorResponse {
          //         code: "INVALID_PASSWORD".to_string(),
          //         message: "Invalid password".to_string(),
          //     };
          //     (StatusCode::BAD_REQUEST, Json(body)).into_response()
          // }
          // SystemError::InvalidCodeValidation => {
          //     let body = ApiErrorResponse {
          //         code: "INVALID_CODE_VALIDATION".to_string(),
          //         message: "Invalid code validation".to_string(),
          //     };
          //     (StatusCode::BAD_REQUEST, Json(body)).into_response()
          // }
          // SystemError::UserNotFound => {
          //     let body = ApiErrorResponse {
          //         code: "USER_NOT_FOUND".to_string(),
          //         message: "User not found".to_string(),
          //     };
          //     (StatusCode::NOT_FOUND, Json(body)).into_response()
          // }
          // SystemError::LoginFailed => {
          //     let body = ApiErrorResponse {
          //         code: "LOGIN_FAILED".to_string(),
          //         message: "Login failed".to_string(),
          //     };
          //     (StatusCode::UNAUTHORIZED, Json(body)).into_response()
          // }
          // SystemError::ActivationFailed => {
          //     let body = ApiErrorResponse {
          //         code: "ACTIVATION_FAILED".to_string(),
          //         message: "Activation failed".to_string(),
          //     };
          //     (StatusCode::UNAUTHORIZED, Json(body)).into_response()
          // }
          // SystemError::Common(CommonApplicationError::Infrastructure) => {
          //     StatusCode::INTERNAL_SERVER_ERROR.into_response()
          // }
          // SystemError::Common(CommonApplicationError::Unauthorized) => {
          //     StatusCode::UNAUTHORIZED.into_response()
          // }
    }
}

use crate::http::common::errors::api_error_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use iam::{
    application::errors::error_codes::{
        IAM_ACCOUNT_REPOSITORY_ERROR, IAM_CANNOT_AUTHENTICATE, IAM_LOGIN_FAILED,
        IAM_PASSWORD_TOO_SHORT, IAM_TOKEN_GENERATOR_ERROR,
    },
    domain::errors::error_codes::{
        IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_INVALID_VERIFICATION, IAM_ACCOUNT_NOT_FOUND,
        IAM_ACCOUNT_USERNAME_ALREADY_EXISTS, IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT,
        IAM_INVALID_ACCOUNT_STATUS_TRANSITION, IAM_INVALID_CODE_VALIDATION, IAM_INVALID_EMAIL,
        IAM_INVALID_HASHED_PASSWORD, IAM_INVALID_USERNAME,
    },
};
use shared::error::SystemError;

pub fn map_application_error(error: SystemError) -> Response {
    let (status, code, message) = match error {
        SystemError::Domain(err)
        | SystemError::Application(err)
        | SystemError::Infrastructure(err) => {
            let code = err.code();
            let message = err.message();
            let status = status_from_error_code(code);
            (status, code, message)
        }
    };

    let body = ApiErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    };

    (status, Json(body)).into_response()
}

fn status_from_error_code(code: &str) -> StatusCode {
    match code {
        IAM_ACCOUNT_EMAIL_ALREADY_EXISTS | IAM_ACCOUNT_USERNAME_ALREADY_EXISTS => {
            StatusCode::CONFLICT
        }
        IAM_ACCOUNT_NOT_FOUND => StatusCode::NOT_FOUND,
        IAM_LOGIN_FAILED | IAM_CANNOT_AUTHENTICATE => StatusCode::UNAUTHORIZED,
        IAM_INVALID_ACCOUNT_ID
        | IAM_INVALID_ACCOUNT_ID_FORMAT
        | IAM_INVALID_ACCOUNT_STATUS_TRANSITION
        | IAM_INVALID_CODE_VALIDATION
        | IAM_INVALID_EMAIL
        | IAM_INVALID_HASHED_PASSWORD
        | IAM_INVALID_USERNAME
        | IAM_PASSWORD_TOO_SHORT
        | IAM_ACCOUNT_INVALID_VERIFICATION => StatusCode::BAD_REQUEST,
        IAM_ACCOUNT_REPOSITORY_ERROR | IAM_TOKEN_GENERATOR_ERROR => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

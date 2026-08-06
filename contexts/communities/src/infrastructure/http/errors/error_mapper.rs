use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared::{error::SystemError, infrastructure::http::api_error_response::ApiErrorResponse};

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
        // COMMUNITIES_SLUG_ALREADY_EXISTS => StatusCode::CONFLICT,
        // COMMUNITIES_INVALID_COMMUNITY_NAME
        // | COMMUNITIES_INVALID_COMMUNITY_SLUG
        // | IAM_INVALID_ACCOUNT_ID
        // | IAM_INVALID_ACCOUNT_ID_FORMAT => StatusCode::BAD_REQUEST,
        // COMMUNITIES_REPOSITORY_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

use crate::{
    application::errors::error_codes::{
        IAM_ACCOUNT_REPOSITORY_ERROR, IAM_CANNOT_AUTHENTICATE, IAM_LOGIN_FAILED,
        IAM_TOKEN_GENERATOR_ERROR,
    },
    domain::errors::error_codes::{
        IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_INVALID_VERIFICATION, IAM_ACCOUNT_NOT_FOUND,
        IAM_ACCOUNT_USERNAME_ALREADY_EXISTS, IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT,
        IAM_INVALID_ACCOUNT_STATUS_TRANSITION, IAM_INVALID_CODE_VALIDATION, IAM_INVALID_EMAIL,
        IAM_INVALID_HASHED_PASSWORD, IAM_INVALID_USERNAME, IAM_PASSWORD_TOO_SHORT,
    },
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
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

#[cfg(test)]
mod tests {
    use super::*;
    use shared::error::{ArchitectureLayer, ErrorLayout};
    use std::fmt;

    #[derive(Debug)]
    struct ErrorWithCode(&'static str);

    impl fmt::Display for ErrorWithCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for ErrorWithCode {}

    impl ErrorLayout for ErrorWithCode {
        fn architecture_layer(&self) -> ArchitectureLayer {
            ArchitectureLayer::Domain
        }
        fn code(&self) -> &'static str {
            self.0
        }
        fn message(&self) -> &'static str {
            "message"
        }
    }

    fn status_for(code: &'static str) -> StatusCode {
        map_application_error(SystemError::from(ErrorWithCode(code))).status()
    }

    #[test]
    fn already_existing_account_maps_to_conflict() {
        for code in [
            IAM_ACCOUNT_EMAIL_ALREADY_EXISTS,
            IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
        ] {
            assert_eq!(status_for(code), StatusCode::CONFLICT, "{code}");
        }
    }

    #[test]
    fn account_not_found_maps_to_not_found() {
        assert_eq!(status_for(IAM_ACCOUNT_NOT_FOUND), StatusCode::NOT_FOUND);
    }

    #[test]
    fn failed_authentication_maps_to_unauthorized() {
        for code in [IAM_LOGIN_FAILED, IAM_CANNOT_AUTHENTICATE] {
            assert_eq!(status_for(code), StatusCode::UNAUTHORIZED, "{code}");
        }
    }

    #[test]
    fn invalid_input_maps_to_bad_request() {
        for code in [
            IAM_INVALID_ACCOUNT_ID,
            IAM_INVALID_ACCOUNT_ID_FORMAT,
            IAM_INVALID_ACCOUNT_STATUS_TRANSITION,
            IAM_INVALID_CODE_VALIDATION,
            IAM_INVALID_EMAIL,
            IAM_INVALID_HASHED_PASSWORD,
            IAM_INVALID_USERNAME,
            IAM_PASSWORD_TOO_SHORT,
            IAM_ACCOUNT_INVALID_VERIFICATION,
        ] {
            assert_eq!(status_for(code), StatusCode::BAD_REQUEST, "{code}");
        }
    }

    #[test]
    fn technical_and_unknown_errors_map_to_internal_server_error() {
        for code in [
            IAM_ACCOUNT_REPOSITORY_ERROR,
            IAM_TOKEN_GENERATOR_ERROR,
            "UNKNOWN",
        ] {
            assert_eq!(
                status_for(code),
                StatusCode::INTERNAL_SERVER_ERROR,
                "{code}"
            );
        }
    }
}

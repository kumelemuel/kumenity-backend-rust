use crate::{
    application::errors::error_codes::COMMUNITIES_SLUG_ALREADY_EXISTS,
    domain::errors::error_codes::{
        COMMUNITIES_INVALID_COMMUNITY_NAME, COMMUNITIES_INVALID_COMMUNITY_SLUG,
    },
    infrastructure::http::{
        codes::{IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT},
        unexpected::unexpected,
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
        SystemError::Unexpected(fault) => return unexpected(&fault),
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
        COMMUNITIES_SLUG_ALREADY_EXISTS => StatusCode::CONFLICT,
        COMMUNITIES_INVALID_COMMUNITY_NAME
        | COMMUNITIES_INVALID_COMMUNITY_SLUG
        | IAM_INVALID_ACCOUNT_ID
        | IAM_INVALID_ACCOUNT_ID_FORMAT => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::http::testing::{D7_BODY, body_of};
    use shared::{
        Fault,
        error::{ArchitectureLayer, ErrorLayout},
    };
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
    fn duplicated_slug_maps_to_conflict() {
        assert_eq!(
            status_for(COMMUNITIES_SLUG_ALREADY_EXISTS),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn invalid_input_maps_to_bad_request() {
        for code in [
            COMMUNITIES_INVALID_COMMUNITY_NAME,
            COMMUNITIES_INVALID_COMMUNITY_SLUG,
            IAM_INVALID_ACCOUNT_ID,
            IAM_INVALID_ACCOUNT_ID_FORMAT,
        ] {
            assert_eq!(status_for(code), StatusCode::BAD_REQUEST, "{code}");
        }
    }

    #[test]
    fn unknown_errors_map_to_internal_server_error() {
        assert_eq!(status_for("UNKNOWN"), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// The legacy arm (until R19) delegates to `unexpected`: same D7 body.
    #[test]
    fn unexpected_system_error_maps_to_the_d7_body() {
        let fault = Fault::new("connection refused: db.internal:5432");

        let response = map_application_error(SystemError::Unexpected(fault));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

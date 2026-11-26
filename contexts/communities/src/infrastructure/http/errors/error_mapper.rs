use crate::infrastructure::http::unexpected::unexpected;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{error::SystemError, infrastructure::http::api_error_response::ApiErrorResponse};

/// Legacy mapper, deleted in R19. Since R17 no endpoint of communities uses
/// it: `POST /communities/create` has its exhaustive `fold`, and no error of
/// this context reaches here with a client code, so everything is a 500.
pub fn map_application_error(error: SystemError) -> Response {
    let (code, message) = match error {
        SystemError::Unexpected(fault) => return unexpected(&fault),
        SystemError::Domain(err)
        | SystemError::Application(err)
        | SystemError::Infrastructure(err) => (err.code(), err.message()),
    };

    let body = ApiErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
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

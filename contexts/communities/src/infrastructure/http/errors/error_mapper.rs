use crate::{
    application::errors::error_codes::COMMUNITIES_SLUG_ALREADY_EXISTS,
    domain::errors::error_codes::{
        COMMUNITIES_INVALID_COMMUNITY_NAME, COMMUNITIES_INVALID_COMMUNITY_SLUG,
    },
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::domain::errors::error_codes::{IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT};
use shared::{
    Fault, error::SystemError, infrastructure::http::api_error_response::ApiErrorResponse,
};

pub fn map_application_error(error: SystemError) -> Response {
    let (status, code, message) = match error {
        SystemError::Unexpected(fault) => return unexpected(SystemError::UNEXPECTED_CODE, fault),
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

/// Mensaje interino del 500 por `Fault` (D7), hasta Problem Details (R30).
const INTERNAL_ERROR_MESSAGE: &str =
    "We couldn't complete your request right now. Please try again.";

/// Árbol B: 500 genérico, sin exponer el detalle del `Fault` al cliente.
fn unexpected(code: &'static str, fault: Fault) -> Response {
    // INTERINO hasta R26 (ERR-4.7): todavía no hay `FaultReporter` cableado.
    // Para que el 500 no sea silencioso (ERR-1.4), el detalle va a stderr, donde
    // hoy se registran los errores. R26/R30 lo reemplazan por
    // `unexpected(fault, reporter)` con `StderrFaultReporterAdapter`.
    eprintln!("communities: unexpected fault: {}", fault.detail());

    let body = ApiErrorResponse {
        code: code.to_string(),
        message: INTERNAL_ERROR_MESSAGE.to_string(),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
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

    /// Cuerpo de D7: lo único que ve el cliente ante un `Fault`.
    const D7_BODY: &str = r#"{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}"#;

    fn body_of(response: Response) -> String {
        use std::{
            future::Future,
            pin::pin,
            task::{Context, Poll, Waker},
        };
        let mut future = pin!(axum::body::to_bytes(response.into_body(), usize::MAX));
        let Poll::Ready(bytes) = future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        else {
            panic!("el cuerpo de un Json está completo: no puede quedar pendiente");
        };
        String::from_utf8(bytes.unwrap().to_vec()).unwrap()
    }

    #[test]
    fn unexpected_fault_maps_to_internal_server_error() {
        let fault = Fault::new("connection refused: db.internal:5432");

        let response = map_application_error(SystemError::Unexpected(fault));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

use super::codes::{INTERNAL_ERROR, INTERNAL_ERROR_MESSAGE};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{Fault, infrastructure::http::api_error_response::ApiErrorResponse};

/// Tree B (ERR-1): generic 500 (D7), without exposing the `Fault` detail to
/// the client. Used by the `fold` of `CreateCommunity`.
pub fn unexpected(fault: &Fault) -> Response {
    // INTERIM until R26 (ERR-4.7): there is no `FaultReporter` wired yet. So
    // that the 500 is not silent (ERR-1.4), the detail goes to stderr, where
    // errors are recorded today. R26/R30 replace it with
    // `unexpected(fault, reporter)` and `StderrFaultReporterAdapter`.
    eprintln!("communities: unexpected fault: {}", fault.detail());

    let body = ApiErrorResponse {
        code: INTERNAL_ERROR.to_string(),
        message: INTERNAL_ERROR_MESSAGE.to_string(),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::http::testing::{D7_BODY, body_of};

    #[test]
    fn unexpected_fault_maps_to_internal_server_error() {
        let fault = Fault::new("connection refused: db.internal:5432");

        let response = unexpected(&fault);

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

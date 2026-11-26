use crate::{
    application::{
        errors::not_found::NotFound,
        ports::inbound::identify_account_use_case::IdentifyAccountUseCase,
        queries::identify_account::{IdentifyAccountError, IdentifyAccountQuery},
    },
    infrastructure::http::{
        codes, requests::identify::IdentifyRequest, responses::identified::IdentifiedResponse,
        unexpected::unexpected,
    },
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{Failure, Fault, infrastructure::http::api_error_response::ApiErrorResponse};
use std::sync::Arc;

/// `POST /auth/identify`.
pub async fn identify(
    State(use_case): State<Arc<dyn IdentifyAccountUseCase>>,
    Json(request): Json<IdentifyRequest>,
) -> Response {
    // Exhaustive `fold` of the contract into the transport (ERR-4.5).
    match use_case.execute(IdentifyAccountQuery::from(request)) {
        Ok(found) => (StatusCode::OK, Json(IdentifiedResponse::from(found))).into_response(),
        Err(Failure::Expected(error)) => match error {
            IdentifyAccountError::NotFound(leaf) => not_found(leaf),
        },
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

/// One helper per leaf (ERR-4). A resource the handler does not emit is a
/// bug, not a client error: tree B.
fn not_found(leaf: NotFound) -> Response {
    match leaf.resource {
        "account" => error(
            StatusCode::NOT_FOUND,
            codes::IAM_ACCOUNT_NOT_FOUND,
            codes::IAM_ACCOUNT_NOT_FOUND_MESSAGE,
        ),
        resource => unexpected(&Fault::new(format!(
            "identify: unmapped not found resource `{resource}`"
        ))),
    }
}

/// Body `{code, message}` of today (until Problem Details, R30).
fn error(status: StatusCode, code: &str, message: &str) -> Response {
    let body = ApiErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    };
    (status, Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::queries::identify_account::IdentifyAccountResponse,
        infrastructure::http::testing::{D7_BODY, body_of, ready},
    };

    type Outcome = Result<IdentifyAccountResponse, Failure<IdentifyAccountError>>;

    /// Always answers the same outcome.
    struct StubIdentifyAccountUseCase(fn() -> Outcome);

    impl IdentifyAccountUseCase for StubIdentifyAccountUseCase {
        fn execute(&self, _query: IdentifyAccountQuery) -> Outcome {
            (self.0)()
        }
    }

    fn respond(outcome: fn() -> Outcome) -> Response {
        let use_case: Arc<dyn IdentifyAccountUseCase> =
            Arc::new(StubIdentifyAccountUseCase(outcome));
        let request = IdentifyRequest {
            identify: "ana".to_string(),
        };
        ready(identify(State(use_case), Json(request)))
    }

    #[test]
    fn an_identified_account_returns_200_with_username_and_status() {
        let response = respond(|| {
            Ok(IdentifyAccountResponse {
                username: "ana".to_string(),
                status: "active".to_string(),
            })
        });

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_of(response), r#"{"username":"ana","status":"active"}"#);
    }

    #[test]
    fn an_account_not_found_returns_404_iam_account_not_found() {
        let response = respond(|| {
            Err(Failure::expected(NotFound {
                resource: "account",
            }))
        });

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            body_of(response),
            r#"{"code":"IAM_ACCOUNT_NOT_FOUND","message":"No account was found with the provided information."}"#
        );
    }

    #[test]
    fn a_fault_returns_500_with_the_d7_body() {
        let response = respond(|| Err(Failure::Unexpected(Fault::new("db.internal:5432 refused"))));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_not_found_resource_returns_500() {
        let response = respond(|| Err(Failure::expected(NotFound { resource: "token" })));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

use crate::{
    application::{
        commands::verify_account::{VerifyAccountCommand, VerifyAccountError},
        errors::{
            invalid_verification::InvalidVerification, not_found::NotFound, validation::Validation,
        },
        ports::inbound::verify_account_use_case::VerifyAccountUseCase,
    },
    infrastructure::http::{codes, requests::verify::VerifyRequest, unexpected::unexpected},
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{Failure, Fault, infrastructure::http::api_error_response::ApiErrorResponse};
use std::sync::Arc;

/// `POST /auth/verify`.
pub async fn verify(
    State(use_case): State<Arc<dyn VerifyAccountUseCase>>,
    Json(request): Json<VerifyRequest>,
) -> Response {
    // Exhaustive `fold` of the contract into the transport (ERR-4.5).
    match use_case.execute(VerifyAccountCommand::from(request)) {
        // Today's body for a success: `null`.
        Ok(()) => (StatusCode::OK, Json(())).into_response(),
        Err(Failure::Expected(error)) => match error {
            VerifyAccountError::NotFound(leaf) => not_found(leaf),
            VerifyAccountError::Validation(leaf) => validation(leaf),
            VerifyAccountError::InvalidVerification(leaf) => invalid_verification(leaf),
        },
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

/// One helper per leaf (ERR-4). A resource or field the handler does not
/// emit is a bug, not a client error: tree B.
fn not_found(leaf: NotFound) -> Response {
    match leaf.resource {
        "account" => error(
            StatusCode::NOT_FOUND,
            codes::IAM_ACCOUNT_NOT_FOUND,
            codes::IAM_ACCOUNT_NOT_FOUND_MESSAGE,
        ),
        resource => unexpected(&unmapped("not found resource", resource)),
    }
}

fn validation(leaf: Validation) -> Response {
    match leaf.field {
        "code" => error(
            StatusCode::BAD_REQUEST,
            codes::IAM_INVALID_CODE_VALIDATION,
            codes::IAM_INVALID_CODE_VALIDATION_MESSAGE,
        ),
        field => unexpected(&unmapped("validation field", field)),
    }
}

fn invalid_verification(_leaf: InvalidVerification) -> Response {
    error(
        StatusCode::BAD_REQUEST,
        codes::IAM_ACCOUNT_INVALID_VERIFICATION,
        codes::IAM_ACCOUNT_INVALID_VERIFICATION_MESSAGE,
    )
}

fn unmapped(kind: &str, name: &str) -> Fault {
    Fault::new(format!("verify: unmapped {kind} `{name}`"))
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
    use crate::infrastructure::http::testing::{D7_BODY, body_of, ready};

    /// Always answers the same outcome.
    struct StubVerifyAccountUseCase(fn() -> Result<(), Failure<VerifyAccountError>>);

    impl VerifyAccountUseCase for StubVerifyAccountUseCase {
        fn execute(
            &self,
            _command: VerifyAccountCommand,
        ) -> Result<(), Failure<VerifyAccountError>> {
            (self.0)()
        }
    }

    fn respond(outcome: fn() -> Result<(), Failure<VerifyAccountError>>) -> Response {
        let use_case: Arc<dyn VerifyAccountUseCase> = Arc::new(StubVerifyAccountUseCase(outcome));
        let request = VerifyRequest {
            email: "ana@example.com".to_string(),
            validation_code: 123_456,
        };
        ready(verify(State(use_case), Json(request)))
    }

    fn assert_body(response: Response, status: StatusCode, code: &str, message: &str) {
        assert_eq!(response.status(), status);
        assert_eq!(
            body_of(response),
            format!(r#"{{"code":"{code}","message":"{message}"}}"#)
        );
    }

    #[test]
    fn a_verified_account_returns_200_with_a_null_body() {
        let response = respond(|| Ok(()));

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_of(response), "null");
    }

    #[test]
    fn a_fault_returns_500_with_the_d7_body() {
        let response = respond(|| Err(Failure::Unexpected(Fault::new("db.internal:5432 refused"))));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_account_not_found_returns_404_iam_account_not_found() {
        assert_body(
            respond(|| {
                Err(Failure::expected(NotFound {
                    resource: "account",
                }))
            }),
            StatusCode::NOT_FOUND,
            "IAM_ACCOUNT_NOT_FOUND",
            "No account was found with the provided information.",
        );
    }

    #[test]
    fn a_code_validation_returns_400_iam_invalid_code_validation() {
        assert_body(
            respond(|| Err(Failure::expected(Validation { field: "code" }))),
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_CODE_VALIDATION",
            "The verification code is invalid.",
        );
    }

    #[test]
    fn an_invalid_verification_returns_400_iam_account_invalid_verification() {
        assert_body(
            respond(|| Err(Failure::expected(InvalidVerification))),
            StatusCode::BAD_REQUEST,
            "IAM_ACCOUNT_INVALID_VERIFICATION",
            "Verification process is invalid.",
        );
    }

    #[test]
    fn an_unmapped_not_found_resource_returns_500() {
        let response = respond(|| Err(Failure::expected(NotFound { resource: "token" })));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_validation_field_returns_500() {
        let response = respond(|| Err(Failure::expected(Validation { field: "email" })));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

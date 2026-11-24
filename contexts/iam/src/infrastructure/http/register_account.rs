use crate::{
    application::{
        commands::register_account::{RegisterAccountCommand, RegisterAccountError},
        errors::{conflict::Conflict, validation::Validation},
        ports::inbound::register_account_use_case::RegisterAccountUseCase,
    },
    infrastructure::http::{
        codes, requests::sign_up::SignUpRequest, responses::signed_up::SignedUpResponse,
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

/// `POST /auth/register`.
pub async fn register(
    State(use_case): State<Arc<dyn RegisterAccountUseCase>>,
    Json(request): Json<SignUpRequest>,
) -> Response {
    // Exhaustive `fold` of the contract into the transport (ERR-4.5).
    match use_case.execute(RegisterAccountCommand::from(request)) {
        Ok(result) => (StatusCode::CREATED, Json(SignedUpResponse::from(result))).into_response(),
        Err(Failure::Expected(error)) => match error {
            RegisterAccountError::Validation(leaf) => validation(leaf),
            RegisterAccountError::Conflict(leaf) => conflict(leaf),
        },
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

/// One helper per leaf (ERR-4). A field the handler does not emit is a bug,
/// not a client error: tree B.
fn validation(leaf: Validation) -> Response {
    let (code, message) = match leaf.field {
        "password" => (
            codes::IAM_PASSWORD_TOO_SHORT,
            codes::IAM_PASSWORD_TOO_SHORT_MESSAGE,
        ),
        "username" => (
            codes::IAM_INVALID_USERNAME,
            codes::IAM_INVALID_USERNAME_MESSAGE,
        ),
        "email" => (codes::IAM_INVALID_EMAIL, codes::IAM_INVALID_EMAIL_MESSAGE),
        field => return unexpected(&unmapped("validation field", field)),
    };
    error(StatusCode::BAD_REQUEST, code, message)
}

fn conflict(leaf: Conflict) -> Response {
    let (code, message) = match leaf.resource {
        "email" => (
            codes::IAM_ACCOUNT_EMAIL_ALREADY_EXISTS,
            codes::IAM_ACCOUNT_EMAIL_ALREADY_EXISTS_MESSAGE,
        ),
        "username" => (
            codes::IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
            codes::IAM_ACCOUNT_USERNAME_ALREADY_EXISTS_MESSAGE,
        ),
        resource => return unexpected(&unmapped("conflict resource", resource)),
    };
    error(StatusCode::CONFLICT, code, message)
}

fn unmapped(kind: &str, name: &str) -> Fault {
    Fault::new(format!("register: unmapped {kind} `{name}`"))
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
        application::commands::register_account::RegisterAccountResponse,
        infrastructure::http::testing::{D7_BODY, body_of, ready},
    };

    /// Always answers the same outcome.
    struct StubRegisterAccountUseCase(fn() -> Failure<RegisterAccountError>);

    impl RegisterAccountUseCase for StubRegisterAccountUseCase {
        fn execute(
            &self,
            _command: RegisterAccountCommand,
        ) -> Result<RegisterAccountResponse, Failure<RegisterAccountError>> {
            Err((self.0)())
        }
    }

    fn respond(outcome: fn() -> Failure<RegisterAccountError>) -> Response {
        let use_case: Arc<dyn RegisterAccountUseCase> =
            Arc::new(StubRegisterAccountUseCase(outcome));
        let request = SignUpRequest {
            username: "ana".to_string(),
            email: "ana@example.com".to_string(),
            password: "s3cret-password".to_string(),
        };
        ready(register(State(use_case), Json(request)))
    }

    #[test]
    fn a_fault_returns_500_with_the_d7_body() {
        let response = respond(|| Failure::Unexpected(Fault::new("db.internal:5432 refused")));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_validation_field_returns_500() {
        let response = respond(|| Failure::expected(Validation { field: "nickname" }));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_conflict_resource_returns_500() {
        let response = respond(|| Failure::expected(Conflict { resource: "phone" }));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }
}

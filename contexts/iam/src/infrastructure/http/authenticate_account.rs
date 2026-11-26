use crate::{
    application::{
        commands::authenticate_account::{AuthenticateAccountCommand, AuthenticateAccountError},
        errors::{cannot_authenticate::CannotAuthenticate, login_failed::LoginFailed},
        ports::inbound::authenticate_account_use_case::AuthenticateAccountUseCase,
    },
    infrastructure::http::{
        codes, requests::sign_in::SignInRequest, responses::signed_in::SignedInResponse,
        unexpected::unexpected,
    },
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{Failure, infrastructure::http::api_error_response::ApiErrorResponse};
use std::sync::Arc;

/// `POST /auth/authenticate`.
pub async fn authenticate(
    State(use_case): State<Arc<dyn AuthenticateAccountUseCase>>,
    Json(request): Json<SignInRequest>,
) -> Response {
    // Exhaustive `fold` of the contract into the transport (ERR-4.5).
    match use_case.execute(AuthenticateAccountCommand::from(request)) {
        Ok(response) => (StatusCode::OK, Json(SignedInResponse::from(response))).into_response(),
        Err(Failure::Expected(error)) => match error {
            AuthenticateAccountError::LoginFailed(leaf) => login_failed(leaf),
            AuthenticateAccountError::CannotAuthenticate(leaf) => cannot_authenticate(leaf),
        },
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

/// One helper per leaf (ERR-4).
fn login_failed(_leaf: LoginFailed) -> Response {
    error(
        StatusCode::UNAUTHORIZED,
        codes::IAM_LOGIN_FAILED,
        codes::IAM_LOGIN_FAILED_MESSAGE,
    )
}

fn cannot_authenticate(_leaf: CannotAuthenticate) -> Response {
    error(
        StatusCode::UNAUTHORIZED,
        codes::IAM_CANNOT_AUTHENTICATE,
        codes::IAM_CANNOT_AUTHENTICATE_MESSAGE,
    )
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
        application::commands::authenticate_account::AuthenticateAccountResponse,
        infrastructure::http::testing::{D7_BODY, body_of, ready},
    };
    use shared::Fault;

    type Outcome = Result<AuthenticateAccountResponse, Failure<AuthenticateAccountError>>;

    /// Always answers the same outcome.
    struct StubAuthenticateAccountUseCase(fn() -> Outcome);

    impl AuthenticateAccountUseCase for StubAuthenticateAccountUseCase {
        fn execute(&self, _command: AuthenticateAccountCommand) -> Outcome {
            (self.0)()
        }
    }

    fn respond(outcome: fn() -> Outcome) -> Response {
        let use_case: Arc<dyn AuthenticateAccountUseCase> =
            Arc::new(StubAuthenticateAccountUseCase(outcome));
        let request = SignInRequest {
            username: "ana".to_string(),
            password: "s3cret-password".to_string(),
        };
        ready(authenticate(State(use_case), Json(request)))
    }

    fn assert_body(response: Response, status: StatusCode, code: &str, message: &str) {
        assert_eq!(response.status(), status);
        assert_eq!(
            body_of(response),
            format!(r#"{{"code":"{code}","message":"{message}"}}"#)
        );
    }

    #[test]
    fn an_authenticated_account_returns_200_with_the_token() {
        let response = respond(|| {
            Ok(AuthenticateAccountResponse {
                token: "header.payload.signature".to_string(),
            })
        });

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_of(response), r#"{"token":"header.payload.signature"}"#);
    }

    #[test]
    fn a_fault_returns_500_with_the_d7_body() {
        let response = respond(|| Err(Failure::Unexpected(Fault::new("jwt: clock overflow"))));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn a_login_failed_returns_401_iam_login_failed() {
        assert_body(
            respond(|| Err(Failure::expected(LoginFailed))),
            StatusCode::UNAUTHORIZED,
            "IAM_LOGIN_FAILED",
            "The username or password is incorrect.",
        );
    }

    #[test]
    fn a_cannot_authenticate_returns_401_iam_cannot_authenticate() {
        assert_body(
            respond(|| Err(Failure::expected(CannotAuthenticate))),
            StatusCode::UNAUTHORIZED,
            "IAM_CANNOT_AUTHENTICATE",
            "This account is not allowed to sign in at the moment.",
        );
    }
}

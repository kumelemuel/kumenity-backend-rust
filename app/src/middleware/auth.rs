use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{StatusCode, header::AUTHORIZATION};
use iam::{
    application::{
        ports::inbound::verify_access_token_use_case::VerifyAccessTokenUseCase,
        queries::verify_access_token::{VerifyAccessTokenError, VerifyAccessTokenQuery},
    },
    infrastructure::http::unexpected::unexpected,
};
use shared::{Failure, domain::model::verified_identity::VerifiedIdentity};
use std::sync::Arc;

pub async fn auth_middleware(
    State(verify_access_token): State<Arc<dyn VerifyAccessTokenUseCase>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let Some(raw_token) = token else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    match verify_access_token.execute(VerifyAccessTokenQuery {
        token: raw_token.to_string(),
    }) {
        Ok(response) => {
            request.extensions_mut().insert(VerifiedIdentity {
                account_id: response.account_id,
            });
            next.run(request).await
        }
        Err(Failure::Expected(VerifyAccessTokenError::Unauthenticated(_))) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
        // Tree B: not a 401. `unexpected` answers the 500 of D7 and records
        // the `Fault` (on stderr until R26).
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, middleware, routing::get};
    use http_body_util::BodyExt;
    use iam::application::queries::verify_access_token::VerifyAccessTokenResponse;
    use shared::Fault;
    use tower::ServiceExt;

    /// Body of D7: all the client sees of a `Fault`.
    const D7_BODY: &str = r#"{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}"#;

    /// Always answers a `Fault`, like a verifier with a misconfigured key.
    struct StubVerifyAccessTokenUseCase;

    impl VerifyAccessTokenUseCase for StubVerifyAccessTokenUseCase {
        fn execute(
            &self,
            _query: VerifyAccessTokenQuery,
        ) -> Result<VerifyAccessTokenResponse, Failure<VerifyAccessTokenError>> {
            Err(Failure::Unexpected(Fault::new("jwt: misconfigured key")))
        }
    }

    #[tokio::test]
    async fn a_fault_verifying_the_token_returns_500() {
        let use_case: Arc<dyn VerifyAccessTokenUseCase> = Arc::new(StubVerifyAccessTokenUseCase);
        let app = Router::new()
            .route("/protected", get(|| async { "reached" }))
            .layer(middleware::from_fn_with_state(use_case, auth_middleware));
        let request = Request::builder()
            .uri("/protected")
            .header(AUTHORIZATION, "Bearer any-token")
            .body(Body::empty())
            .expect("request");

        let response = app.oneshot(request).await.expect("response");

        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(String::from_utf8_lossy(&body), D7_BODY);
    }
}

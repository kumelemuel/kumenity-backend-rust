use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{StatusCode, header::AUTHORIZATION};
use iam::application::{
    ports::inbound::verify_access_token_use_case::VerifyAccessTokenUseCase,
    queries::verify_access_token::{VerifyAccessTokenError, VerifyAccessTokenQuery},
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
        // The `Fault` is discarded on purpose, keeping today's 401, until
        // the commit that answers it with a 500 and its test (R10b).
        Err(Failure::Unexpected(_fault)) => StatusCode::UNAUTHORIZED.into_response(),
    }
}

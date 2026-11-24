use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{StatusCode, header::AUTHORIZATION};
use iam::application::ports::outbound::{
    dtos::access_token_payloads::AccessTokenVerdict, token_generator::AccessTokenService,
};
use shared::domain::model::verified_identity::VerifiedIdentity;
use std::sync::Arc;

pub async fn auth_middleware(
    State(tokens): State<Arc<dyn AccessTokenService>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match token {
        None => StatusCode::UNAUTHORIZED.into_response(),
        Some(raw_token) => match tokens.verify(raw_token) {
            Ok(AccessTokenVerdict::Valid { account_id }) => {
                request
                    .extensions_mut()
                    .insert(VerifiedIdentity { account_id });
                next.run(request).await
            }
            Ok(AccessTokenVerdict::Invalid) => StatusCode::UNAUTHORIZED.into_response(),
            // The `Fault` is discarded on purpose, keeping today's 401, until
            // the commit that answers it with a 500 and its test (R10).
            Err(_fault) => StatusCode::UNAUTHORIZED.into_response(),
        },
    }
}

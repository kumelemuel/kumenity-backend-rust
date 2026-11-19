use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{header::AUTHORIZATION, StatusCode};
use shared::application::ports::outbound::token_verifier::TokenVerifier;
use std::sync::Arc;

pub async fn auth_middleware(
    State(verifier): State<Arc<dyn TokenVerifier>>,
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
        Some(raw_token) => match verifier.verify(raw_token) {
            Ok(identity) => {
                request.extensions_mut().insert(identity);
                next.run(request).await
            }
            Err(_) => StatusCode::UNAUTHORIZED.into_response(),
        },
    }
}

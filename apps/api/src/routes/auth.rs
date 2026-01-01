use axum::{Router, routing::post};
use crate::http::iam::handlers::sign_in_handler::sign_in_handler;
use crate::http::iam::handlers::sign_up_handler::sign_up_handler;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(sign_up_handler))
        .route("/sign-in", post(sign_in_handler))
}

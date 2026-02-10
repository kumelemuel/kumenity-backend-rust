use axum::{Router, routing::post};
use crate::http::iam::handlers::identify::identify_handler;
use crate::http::iam::handlers::sign_in::sign_in_handler;
use crate::http::iam::handlers::sign_up::sign_up_handler;
use crate::http::iam::handlers::verify::verify_handler;
use crate::state::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(sign_up_handler))
        .route("/verify", post(verify_handler))
        .route("/identify", post(identify_handler))
        .route("/sign-in", post(sign_in_handler))
}

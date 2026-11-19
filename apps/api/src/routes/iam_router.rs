use crate::{
    http::iam::handlers::{
        identify::identify_handler, sign_in::sign_in_handler, sign_up::sign_up_handler,
        verify::verify_handler,
    },
    state::app::AppState,
};
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(sign_up_handler))
        .route("/verify", post(verify_handler))
        .route("/identify", post(identify_handler))
        .route("/sign-in", post(sign_in_handler))
}

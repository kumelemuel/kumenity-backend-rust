use axum::{Router, routing::post};
use crate::http::communities::handlers::create::create_handler;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_handler))
}

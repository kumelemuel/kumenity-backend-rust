use crate::{http::communities::handlers::create::create_handler, state::app::AppState};
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create_handler))
}

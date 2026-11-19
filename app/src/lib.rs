pub mod config;
mod containers;
mod middleware;
mod server;

use crate::{config::app::AppConfig, containers::app::AppContainer, server::Server};
use axum::Router;

/// Builds the HTTP application with its full composition root.
pub async fn build_app(config: &AppConfig) -> Router {
    let container = AppContainer::build(config).await;
    Server::build(&container)
}

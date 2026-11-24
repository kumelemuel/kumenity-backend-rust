pub mod config;
mod containers;
mod middleware;
mod server;

use crate::{config::app::AppConfig, containers::app::AppContainer, server::Server};
use axum::Router;
use shared::Fault;

/// Builds the HTTP application with its full composition root.
///
/// Fails with the `Fault` of any adapter the composition root has to call
/// before serving (the decoy hash of D6).
pub async fn build_app(config: &AppConfig) -> Result<Router, Fault> {
    let container = AppContainer::build(config).await?;
    Ok(Server::build(&container))
}

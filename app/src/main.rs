mod config;
mod containers;
mod middleware;
mod server;

use crate::{config::app::AppConfig, containers::app::AppContainer, server::Server};

#[tokio::main]
async fn main() {
    let config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });
    let container = AppContainer::build(&config).await;

    let app = Server::build(&container);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

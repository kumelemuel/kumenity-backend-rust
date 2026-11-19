use app::{build_app, config::app::AppConfig};

#[tokio::main]
async fn main() {
    let config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });

    let app = build_app(&config).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

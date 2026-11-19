use api::{build_app, config::jwt::JwtConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_config = JwtConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });

    let app = build_app(jwt_config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

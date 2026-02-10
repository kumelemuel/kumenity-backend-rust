mod http;
mod state;
mod routes;
mod config;
mod middleware;
mod authentication;

use axum::Router;
use std::sync::Arc;
use crate::routes::{iam_router, communities_router};
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;
use crate::authentication::token_validator::JwtValidator;
use crate::config::jwt::JwtConfig;
use crate::state::app::AppState;
use crate::state::communities::CommunitiesState;
use crate::state::iam::IamState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_config = JwtConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });
    let token_generator = Arc::new(JwtTokenGenerator::new(
        jwt_config.secret, jwt_config.expiration_time,
    ));

    let iam_state = IamState::initialize(token_generator.clone());
    let communities_state = CommunitiesState::initialize();

    let state = AppState {
        iam: iam_state,
        communities: communities_state,
        token_validator:  Arc::new(JwtValidator::new(token_generator.clone())),
    };

    let app = Router::new()
        .nest("/auth", iam_router::router())
        .nest("/communities", communities_router::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

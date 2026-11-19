mod authentication;
pub mod config;
mod http;
mod middleware;
mod routes;
mod state;

use crate::{
    authentication::token_validator::JwtValidator,
    config::jwt::JwtConfig,
    routes::{communities_router, iam_router},
    state::{app::AppState, communities::CommunitiesState, iam::IamState},
};
use axum::Router;
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;
use std::sync::Arc;

/// Builds the HTTP application with its full composition root.
pub fn build_app(jwt_config: JwtConfig) -> Router {
    let token_generator = Arc::new(JwtTokenGenerator::new(
        jwt_config.secret,
        jwt_config.expiration_time,
    ));

    let iam_state = IamState::initialize(token_generator.clone());
    let communities_state = CommunitiesState::initialize();

    let state = AppState {
        iam: iam_state,
        communities: communities_state,
        token_validator: Arc::new(JwtValidator::new(token_generator.clone())),
    };

    Router::new()
        .nest("/auth", iam_router::router())
        .nest("/communities", communities_router::router())
        .with_state(state)
}

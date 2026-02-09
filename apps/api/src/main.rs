mod http;
mod state;
mod routes;
mod config;
mod middleware;
mod authentication;

use axum::Router;
use std::sync::Arc;
use communities::application::use_cases::create_community::CreateCommunityUseCase;
use communities::infrastructure::persistence::in_memory::community_repository::InMemoryCommunityRepository;
use crate::state::AppState;
use crate::routes::{iam_router, communities_router};
use iam::application::use_cases::register_account::RegisterAccountUseCase;
use iam::application::use_cases::authenticate_account::AuthenticateAccountUseCase;
use iam::application::use_cases::identify_account::IdentifyAccountUseCase;
use iam::application::use_cases::verify_account::VerifyAccountUseCase;
use iam::infrastructure::persistence::in_memory::account_repository::InMemoryAccountRepository;
use iam::infrastructure::security::password_hasher::argon2_password_hasher::Argon2PasswordHasher;
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;
use crate::authentication::token_validator::JwtValidator;
use crate::config::jwt::JwtConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_config = JwtConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });

    // IAM
    let account_repository = Arc::new(InMemoryAccountRepository::new());
    let password_hasher = Arc::new(Argon2PasswordHasher::new());
    let token_generator = Arc::new(JwtTokenGenerator::new(
        jwt_config.secret, jwt_config.expiration_time,
    ));

    let register_account = RegisterAccountUseCase::new(account_repository.clone(), password_hasher.clone());
    let authenticate_account = AuthenticateAccountUseCase::new(account_repository.clone(), password_hasher.clone(), token_generator.clone());
    let verify_account = VerifyAccountUseCase::new(account_repository.clone());
    let identify_account = IdentifyAccountUseCase::new(account_repository.clone());

    // COMMUNITIES
    let community_repository = Arc::new(InMemoryCommunityRepository::new());

    let create_community = CreateCommunityUseCase::new(community_repository.clone());

    let state = AppState {
        register_account: Arc::new(register_account),
        authenticate_account: Arc::new(authenticate_account),
        verify_account: Arc::new(verify_account),
        identify_account: Arc::new(identify_account),
        token_validator:  Arc::new(JwtValidator::new(token_generator.clone())),
        create_community: Arc::new(create_community),
    };

    let app = Router::new()
        .nest("/auth", iam_router::router())
        .nest("/communities", communities_router::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

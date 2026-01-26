mod http;
mod state;
mod routes;
mod config;

use axum::Router;
use std::sync::Arc;
use crate::state::AppState;
use crate::routes::auth;
use iam::application::use_cases::register_user::RegisterUser;
use iam::application::use_cases::login::Login;
use iam::application::use_cases::validate_user::ValidateUser;
use iam::infrastructure::persistence::in_memory::user_repository::InMemoryUserRepository;
use iam::infrastructure::security::password_hasher::argon2_password_hasher::Argon2PasswordHasher;
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;
use crate::config::jwt::JwtConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_config = JwtConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let password_hasher = Arc::new(Argon2PasswordHasher::new());
    let token_generator = Arc::new(JwtTokenGenerator::new(
        jwt_config.secret, 3600,
    ));

    let register_user = RegisterUser::new(user_repository.clone(), password_hasher.clone());
    let login = Login::new(user_repository.clone(), password_hasher.clone(), token_generator.clone());
    let validate_user = ValidateUser::new(user_repository.clone());

    let state = AppState {
        register_user: Arc::new(register_user),
        login: Arc::new(login),
        validate_user: Arc::new(validate_user),
    };

    let app = Router::new()
        .nest("/auth", auth::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

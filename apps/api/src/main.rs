mod http;
mod state;

use self::http::handlers::register_user_handler::register_user_handler;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;
use iam::application::use_cases::register_user::RegisterUser;
use iam::infrastructure::persistence::in_memory::user_repository::InMemoryUserRepository;
use iam::infrastructure::security::password_hasher::argon2_password_hasher::Argon2PasswordHasher;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_repository = Arc::new(InMemoryUserRepository::new());
    let password_hasher = Arc::new(Argon2PasswordHasher::new());

    let register_user = RegisterUser::new(user_repository.clone(), password_hasher.clone());

    let state = AppState {
        register_user: Arc::new(register_user),
    };

    let app = Router::new()
        .route("/users", post(register_user_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

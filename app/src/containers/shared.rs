use iam::infrastructure::services::jwt_service::jwt_service::JwtService;
use shared::application::ports::outbound::token_verifier::TokenVerifier;
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedContainer {
    pub jwt_service: Arc<dyn TokenVerifier>,
}

impl SharedContainer {
    pub fn initialize(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

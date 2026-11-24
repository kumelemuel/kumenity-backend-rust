use iam::{
    application::ports::outbound::token_generator::AccessTokenService,
    infrastructure::services::jwt_service::jwt_service::JwtService,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedContainer {
    pub jwt_service: Arc<dyn AccessTokenService>,
}

impl SharedContainer {
    pub fn initialize(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

use iam::{
    application::ports::outbound::access_token_service::AccessTokenService,
    infrastructure::services::jwt_access_token::JwtAccessTokenAdapter,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedContainer {
    pub jwt_service: Arc<dyn AccessTokenService>,
}

impl SharedContainer {
    pub fn initialize(jwt_service: Arc<JwtAccessTokenAdapter>) -> Self {
        Self { jwt_service }
    }
}

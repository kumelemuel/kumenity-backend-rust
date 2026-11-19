use crate::{
    config::app::AppConfig,
    containers::{communities::CommunitiesContainer, iam::IamContainer, shared::SharedContainer},
};
use iam::infrastructure::services::jwt_service::jwt_service::JwtService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContainer {
    pub iam: IamContainer,
    pub communities: CommunitiesContainer,
    pub shared: SharedContainer,
}

impl AppContainer {
    pub async fn build(config: &AppConfig) -> Self {
        let jwt_service = Arc::new(JwtService::new(
            config.jwt.secret.clone(),
            config.jwt.expiration_time,
        ));

        let shared = SharedContainer::initialize(jwt_service.clone());
        let iam = IamContainer::initialize(jwt_service);
        let communities = CommunitiesContainer::initialize();

        AppContainer {
            iam,
            communities,
            shared,
        }
    }
}

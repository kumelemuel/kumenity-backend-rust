use crate::{
    config::app::AppConfig,
    containers::{communities::CommunitiesContainer, iam::IamContainer},
};
use iam::infrastructure::services::jwt_access_token::JwtAccessTokenAdapter;
use shared::Fault;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContainer {
    pub iam: IamContainer,
    pub communities: CommunitiesContainer,
}

impl AppContainer {
    pub async fn build(config: &AppConfig) -> Result<Self, Fault> {
        let access_tokens = Arc::new(JwtAccessTokenAdapter::new(
            config.jwt.secret.clone(),
            config.jwt.expiration_time,
        ));

        let iam = IamContainer::initialize(access_tokens)?;
        let communities = CommunitiesContainer::initialize();

        Ok(AppContainer { iam, communities })
    }
}

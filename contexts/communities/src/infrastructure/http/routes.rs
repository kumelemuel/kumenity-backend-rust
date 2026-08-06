use crate::{
    application::ports::inbound::create_community_use_case::CreateCommunityUseCase,
    infrastructure::http::handlers::community_handlers,
};
use axum::{routing::post, Router};
use std::sync::Arc;

pub struct CommunitiesRoutes {
    create: Arc<dyn CreateCommunityUseCase>,
}

impl CommunitiesRoutes {
    pub fn new(create: Arc<dyn CreateCommunityUseCase>) -> Self {
        Self { create }
    }

    pub fn public(&self) -> Router {
        Router::new()
    }

    pub fn protected(&self) -> Router {
        Router::new().route(
            "/communities/create",
            post(community_handlers::create).with_state(self.create.clone()),
        )
    }
}

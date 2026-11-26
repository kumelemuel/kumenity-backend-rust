use crate::{
    application::ports::inbound::create_community_use_case::CreateCommunityUseCase,
    infrastructure::http::create_community,
};
use axum::{Router, routing::post};
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
            post(create_community::create).with_state(self.create.clone()),
        )
    }
}

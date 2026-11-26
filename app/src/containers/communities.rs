use communities::{
    application::{
        commands::create_community::CreateCommunityHandler,
        ports::inbound::create_community_use_case::CreateCommunityUseCase,
    },
    infrastructure::{
        http::routes::CommunitiesRoutes,
        persistence::in_memory::in_memory_community::InMemoryCommunityAdapter,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CommunitiesContainer {
    pub create_community: Arc<dyn CreateCommunityUseCase + Send + Sync>,
}

impl CommunitiesContainer {
    pub fn initialize() -> Self {
        let community_repository = Arc::new(InMemoryCommunityAdapter::new());

        let create_community = CreateCommunityHandler::new(community_repository.clone());

        Self {
            create_community: Arc::new(create_community),
        }
    }

    pub fn routes(&self) -> CommunitiesRoutes {
        CommunitiesRoutes::new(self.create_community.clone())
    }
}

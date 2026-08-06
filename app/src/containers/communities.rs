use communities::{
    application::{
        commands::create_community::create_community::CreateCommunity,
        ports::inbound::create_community_use_case::CreateCommunityUseCase,
    },
    infrastructure::{
        http::routes::CommunitiesRoutes,
        persistence::in_memory::community_repository::InMemoryCommunityRepository,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CommunitiesContainer {
    pub create_community: Arc<dyn CreateCommunityUseCase + Send + Sync>,
}

impl CommunitiesContainer {
    pub fn initialize() -> Self {
        let community_repository = Arc::new(InMemoryCommunityRepository::new());

        let create_community = CreateCommunity::new(community_repository.clone());

        Self {
            create_community: Arc::new(create_community),
        }
    }

    pub fn routes(&self) -> CommunitiesRoutes {
        CommunitiesRoutes::new(self.create_community.clone())
    }
}

use std::sync::Arc;
use communities::application::ports::inbound::community_creation::CommunityCreationPort;
use communities::application::use_cases::create_community::CreateCommunityUseCase;
use communities::infrastructure::persistence::in_memory::community_repository::InMemoryCommunityRepository;

#[derive(Clone)]
pub struct CommunitiesState {
    pub create_community: Arc<dyn CommunityCreationPort + Send + Sync>,
}

impl CommunitiesState {
    pub fn initialize() -> Self {
        let community_repository = Arc::new(InMemoryCommunityRepository::new());

        let create_community = CreateCommunityUseCase::new(community_repository.clone());

        Self {
            create_community: Arc::new(create_community),
        }
    }
}
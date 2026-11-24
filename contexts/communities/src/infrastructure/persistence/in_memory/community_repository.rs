use crate::{
    application::ports::outbound::community_repository::CommunityRepository,
    domain::model::community::{community::Community, community_id::CommunityId},
};
use shared::Fault;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct InMemoryCommunityRepository {
    communities: Arc<Mutex<HashMap<CommunityId, Community>>>,
}

impl Default for InMemoryCommunityRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryCommunityRepository {
    pub fn new() -> Self {
        Self {
            communities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CommunityRepository for InMemoryCommunityRepository {
    fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault> {
        let communities = self.communities.lock().expect("mutex poisoned");
        Ok(communities
            .values()
            .find(|community| community.slug().as_str() == slug)
            .cloned())
    }

    fn save(&self, community: &Community) -> Result<(), Fault> {
        let mut communities = self.communities.lock().expect("mutex poisoned");

        communities.insert(community.id().clone(), community.clone());

        Ok(())
    }
}

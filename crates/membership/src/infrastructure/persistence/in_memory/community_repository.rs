use crate::{
    application::ports::outbound::community_repository::CommunityRepositoryPort,
    domain::{aggregates::community::Community, value_objects::community_id::CommunityId},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

pub struct InMemoryCommunityRepository {
    communities: Arc<Mutex<HashMap<CommunityId, Community>>>,
}

impl InMemoryCommunityRepository {
    pub fn new() -> Self {
        Self {
            communities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    /// Each operation leaves the map consistent (a single read or `insert`),
    /// so a panic in another thread cannot leave it half-written: recovering
    /// the guard is safe and keeps one panic from poisoning every request.
    fn communities(&self) -> MutexGuard<'_, HashMap<CommunityId, Community>> {
        self.communities
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

impl CommunityRepositoryPort for InMemoryCommunityRepository {
    fn find_by_id(&self, id: &str) -> Option<Community> {
        let communities = self.communities();
        communities
            .values()
            .find(|community| community.id().as_uuid().to_string() == id)
            .cloned()
    }

    fn find_by_slug(&self, slug: &str) -> Option<Community> {
        let communities = self.communities();
        communities
            .values()
            .find(|community| community.slug().as_str() == slug)
            .cloned()
    }

    fn save(&self, community: &Community) -> Result<(), String> {
        let mut communities = self.communities();

        communities.insert(community.id().clone(), community.clone());

        Ok(())
    }
}

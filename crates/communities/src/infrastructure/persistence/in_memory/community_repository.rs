use crate::{
    application::{
        errors::community_repository::CommunityRepositoryError,
        ports::outbound::community_repository::CommunityRepositoryPort,
    },
    domain::{aggregates::community::Community, value_objects::community_id::CommunityId},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
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
}

impl CommunityRepositoryPort for InMemoryCommunityRepository {
    fn get_public_list(&self, _query: Option<String>) -> Vec<Community> {
        todo!()
    }

    fn find_by_id(&self, id: &str) -> Option<Community> {
        let communities = self.communities.lock().expect("mutex poisoned");
        communities
            .values()
            .find(|community| community.id().as_uuid().to_string() == id)
            .cloned()
    }

    fn find_by_slug(&self, slug: &str) -> Option<Community> {
        let communities = self.communities.lock().expect("mutex poisoned");
        communities
            .values()
            .find(|community| community.slug().as_str() == slug)
            .cloned()
    }

    fn save(&self, community: &Community) -> Result<(), CommunityRepositoryError> {
        let mut communities = self.communities.lock().expect("mutex poisoned");

        communities.insert(community.id().clone(), community.clone());

        Ok(())
    }
}

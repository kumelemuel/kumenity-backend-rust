use crate::{
    application::ports::outbound::community_repository::CommunityRepository,
    domain::model::community::{community::Community, community_id::CommunityId},
};
use shared::Fault;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

/// Adaptador en memoria de `CommunityRepository`.
///
/// Casi igual a `FakeCommunityAdapter` mientras no haya persistencia real. No
/// se fusionan: este es un adaptador de infraestructura (de producción); aquel,
/// un doble de application (TDD-2).
#[derive(Default)]
pub struct InMemoryCommunityAdapter {
    communities: Mutex<HashMap<CommunityId, Community>>,
}

impl InMemoryCommunityAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    fn store(&self) -> MutexGuard<'_, HashMap<CommunityId, Community>> {
        self.communities
            .lock()
            .expect("invariante: mutex sin envenenar")
    }
}

impl CommunityRepository for InMemoryCommunityAdapter {
    fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault> {
        Ok(self
            .store()
            .values()
            .find(|community| community.slug().as_str() == slug)
            .cloned())
    }

    fn save(&self, community: &Community) -> Result<(), Fault> {
        self.store()
            .insert(community.id().clone(), community.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::community_repository_contract! {
        new_adapter: InMemoryCommunityAdapter::new(),
    }
}

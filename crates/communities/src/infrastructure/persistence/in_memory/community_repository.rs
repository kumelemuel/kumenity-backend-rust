use crate::{
    application::{
        errors::community_repository::CommunityRepositoryError,
        ports::outbound::community_repository::CommunityRepositoryPort,
    },
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
    fn get_public_list(&self, query: Option<String>) -> Vec<Community> {
        let needle = query
            .map(|q| q.trim().to_lowercase())
            .filter(|q| !q.is_empty());

        let communities = self.communities();
        let mut public: Vec<Community> = communities
            .values()
            .filter(|community| community.is_public())
            .filter(|community| {
                needle.as_deref().is_none_or(|needle| {
                    community.name().as_str().to_lowercase().contains(needle)
                        || community.slug().as_str().to_lowercase().contains(needle)
                })
            })
            .cloned()
            .collect();
        public.sort_by(|a, b| a.slug().as_str().cmp(b.slug().as_str()));
        public
    }

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

    fn save(&self, community: &Community) -> Result<(), CommunityRepositoryError> {
        let mut communities = self.communities();

        communities.insert(community.id().clone(), community.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn community(slug: &str, name: &str, public: bool) -> Community {
        Community::dummy_with(slug, name, public)
    }

    fn repository_with(communities: Vec<Community>) -> InMemoryCommunityRepository {
        let repository = InMemoryCommunityRepository::new();
        for community in &communities {
            repository.save(community).unwrap();
        }
        repository
    }

    fn slugs(communities: Vec<Community>) -> Vec<String> {
        communities
            .iter()
            .map(|c| c.slug().as_str().to_string())
            .collect()
    }

    #[test]
    fn keeps_serving_after_a_panic_while_the_lock_was_held() {
        let community = community("rust-lovers", "Rust Lovers", true);
        let repository = repository_with(vec![community.clone()]);

        let communities = Arc::clone(&repository.communities);
        let _ = std::thread::spawn(move || {
            let _guard = communities.lock().unwrap();
            panic!("panic while holding the lock");
        })
        .join();

        assert!(repository.find_by_slug("rust-lovers").is_some());
        assert_eq!(repository.get_public_list(None).len(), 1);
        assert!(repository.save(&community).is_ok());
    }

    #[test]
    fn public_list_is_empty_when_there_are_no_communities() {
        let repository = InMemoryCommunityRepository::new();

        assert!(repository.get_public_list(None).is_empty());
    }

    #[test]
    fn public_list_excludes_private_communities_and_is_sorted_by_slug() {
        let repository = repository_with(vec![
            community("rust-lovers", "Rust Lovers", true),
            community("secret-club", "Secret Club", false),
            community("go-gophers", "Go Gophers", true),
        ]);

        assert_eq!(
            slugs(repository.get_public_list(None)),
            vec!["go-gophers", "rust-lovers"]
        );
    }

    #[test]
    fn public_list_filters_by_trimmed_case_insensitive_name_or_slug() {
        let repository = repository_with(vec![
            community("rust-lovers", "Rust Lovers", true),
            community("crabs", "Ferris Fans", true),
            community("go-gophers", "Go Gophers", true),
            community("rust-secret", "Rust Secret", false),
        ]);

        assert_eq!(
            slugs(repository.get_public_list(Some("  rUsT ".to_string()))),
            vec!["rust-lovers"]
        );
        assert_eq!(
            slugs(repository.get_public_list(Some("FERRIS".to_string()))),
            vec!["crabs"]
        );
    }

    #[test]
    fn blank_query_does_not_filter() {
        let repository = repository_with(vec![
            community("rust-lovers", "Rust Lovers", true),
            community("go-gophers", "Go Gophers", true),
        ]);

        assert_eq!(
            slugs(repository.get_public_list(Some("   ".to_string()))),
            vec!["go-gophers", "rust-lovers"]
        );
    }
}

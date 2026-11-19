use crate::application::{
    commands::list_public_communities::ListPublicCommunities,
    ports::{
        inbound::public_communities_listing::PublicCommunitiesListingPort,
        outbound::community_repository::CommunityRepositoryPort,
    },
    results::public_communities_listed::{CommunityResult, PublicCommunitiesListed},
};
use shared::{application::auth_context::AuthContext, error::SystemError};
use std::sync::Arc;

pub struct ListPublicCommunitiesUseCase {
    community_repository: Arc<dyn CommunityRepositoryPort>,
}

impl ListPublicCommunitiesUseCase {
    pub fn new(community_repository: Arc<dyn CommunityRepositoryPort>) -> Self {
        Self {
            community_repository,
        }
    }
}

impl PublicCommunitiesListingPort for ListPublicCommunitiesUseCase {
    fn execute(
        &self,
        data: ListPublicCommunities,
        _: AuthContext,
    ) -> Result<PublicCommunitiesListed, SystemError> {
        let communities: Vec<CommunityResult> = self
            .community_repository
            .get_public_list(data.query)
            .iter()
            .map(|community| CommunityResult {
                name: community.name().as_str().to_string(),
                slug: community.slug().as_str().to_string(),
            })
            .collect();

        Ok(PublicCommunitiesListed { communities })
    }
}

#[cfg(test)]
mod tests {
    //! Characterization tests: they pin the behavior observed through the
    //! inbound port so the use case can be refactored safely.
    use super::*;
    use crate::{
        application::errors::community_repository::CommunityRepositoryError,
        domain::{
            aggregates::community::Community,
            value_objects::{
                community_id::CommunityId, community_name::CommunityName,
                community_slug::CommunitySlug,
            },
        },
    };
    use iam::domain::value_objects::AccountId;
    use std::sync::Mutex;

    /// Returns a fixed list of communities and records the queries it receives.
    struct SpyCommunityRepositoryAdapter {
        communities: Vec<Community>,
        received_queries: Mutex<Vec<Option<String>>>,
    }

    impl SpyCommunityRepositoryAdapter {
        fn returning(communities: Vec<Community>) -> Self {
            Self {
                communities,
                received_queries: Mutex::new(Vec::new()),
            }
        }

        fn received_queries(&self) -> Vec<Option<String>> {
            self.received_queries.lock().unwrap().clone()
        }
    }

    impl CommunityRepositoryPort for SpyCommunityRepositoryAdapter {
        fn get_public_list(&self, query: Option<String>) -> Vec<Community> {
            self.received_queries.lock().unwrap().push(query);
            self.communities.clone()
        }

        fn find_by_id(&self, _id: &str) -> Option<Community> {
            None
        }

        fn find_by_slug(&self, _slug: &str) -> Option<Community> {
            None
        }

        fn save(&self, _community: &Community) -> Result<(), CommunityRepositoryError> {
            Ok(())
        }
    }

    fn community(slug: &str, name: &str, public: bool) -> Community {
        Community::create(
            CommunityId::generate(),
            AccountId::generate(),
            CommunitySlug::new(slug.to_string()).unwrap(),
            CommunityName::new(name.to_string()).unwrap(),
            public,
        )
    }

    fn auth() -> AuthContext {
        AuthContext {
            account_id: AccountId::generate().as_uuid().to_string(),
        }
    }

    fn list(query: Option<&str>) -> ListPublicCommunities {
        ListPublicCommunities {
            query: query.map(str::to_string),
        }
    }

    #[test]
    fn returns_empty_list_when_repository_has_no_communities() {
        let repo = Arc::new(SpyCommunityRepositoryAdapter::returning(vec![]));
        let use_case = ListPublicCommunitiesUseCase::new(repo);

        let result = use_case.execute(list(None), auth()).unwrap();

        assert!(result.communities.is_empty());
    }

    #[test]
    fn returns_name_and_slug_of_each_community_in_repository_order() {
        let repo = Arc::new(SpyCommunityRepositoryAdapter::returning(vec![
            community("rust-lovers", "Rust Lovers", true),
            community("go-gophers", "Go Gophers", true),
        ]));
        let use_case = ListPublicCommunitiesUseCase::new(repo);

        let result = use_case.execute(list(None), auth()).unwrap();

        let listed: Vec<(&str, &str)> = result
            .communities
            .iter()
            .map(|c| (c.slug.as_str(), c.name.as_str()))
            .collect();
        assert_eq!(
            listed,
            vec![("rust-lovers", "Rust Lovers"), ("go-gophers", "Go Gophers")]
        );
    }

    #[test]
    fn passes_the_query_through_to_the_repository_unchanged() {
        let repo = Arc::new(SpyCommunityRepositoryAdapter::returning(vec![]));
        let use_case = ListPublicCommunitiesUseCase::new(repo.clone());

        use_case.execute(list(Some("  Rust ")), auth()).unwrap();
        use_case.execute(list(None), auth()).unwrap();

        assert_eq!(
            repo.received_queries(),
            vec![Some("  Rust ".to_string()), None]
        );
    }

    #[test]
    fn does_not_filter_by_visibility_itself_and_relies_on_the_repository() {
        let repo = Arc::new(SpyCommunityRepositoryAdapter::returning(vec![community(
            "secret-club",
            "Secret Club",
            false,
        )]));
        let use_case = ListPublicCommunitiesUseCase::new(repo);

        let result = use_case.execute(list(None), auth()).unwrap();

        assert_eq!(result.communities.len(), 1);
    }
}

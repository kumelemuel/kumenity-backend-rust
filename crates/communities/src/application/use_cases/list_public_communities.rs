use std::sync::Arc;
use shared::application::auth_context::AuthContext;
use crate::application::commands::list_public_communities::ListPublicCommunities;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::inbound::public_communities_listing::PublicCommunitiesListingPort;
use crate::application::ports::outbound::community_repository::CommunityRepositoryPort;
use crate::application::results::public_communities_listed::{CommunityResult, PublicCommunitiesListed};

pub struct ListPublicCommunitiesUseCase {
    community_repository: Arc<dyn CommunityRepositoryPort>,
}

impl ListPublicCommunitiesUseCase {
    pub fn new(community_repository: Arc<dyn CommunityRepositoryPort>) -> Self {
        Self { community_repository }
    }
}

impl PublicCommunitiesListingPort for ListPublicCommunitiesUseCase {
    fn execute(&self, data: ListPublicCommunities, _: AuthContext) -> Result<PublicCommunitiesListed, ApplicationError> {

        let communities: Vec<CommunityResult> = self.community_repository.get_public_list(data.query).iter().map(|community| CommunityResult {
            name: community.name().as_str().to_string(),
            slug: community.slug().as_str().to_string(),
        }).collect();

        Ok( PublicCommunitiesListed { communities })

    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;
//     use iam::domain::value_objects::AccountId;
//     use shared::application::auth_context::AuthContext;
//     use shared::application::common_application_error::CommonApplicationError;
//     use crate::application::commands::create_community::CreateCommunity;
//     use crate::application::errors::application_error::ApplicationError;
//     use crate::application::ports::inbound::community_creation::CommunityCreationPort;
//     use crate::application::ports::outbound::community_repository::test_utils::FakeCommunityRepository;
//     use crate::application::use_cases::create_community::CreateCommunityUseCase;
//
//     fn valid_auth_context() -> AuthContext {
//         AuthContext {
//             account_id: AccountId::generate().as_uuid().to_string(),
//         }
//     }
//
//     fn valid_input() -> CreateCommunity {
//         CreateCommunity {
//             slug: "Community-Test".to_string(),
//             name: "Community Test".to_string(),
//             is_public: true,
//         }
//     }
//
//     #[test]
//     fn create_community_successfully() {
//         let repo = Arc::new(FakeCommunityRepository::success());
//
//         let use_case = CreateCommunityUseCase::new(repo);
//
//         let result = use_case.execute(valid_input(), valid_auth_context());
//
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn fails_when_name_is_invalid() {
//         let repo = Arc::new(FakeCommunityRepository::success());
//
//         let use_case = CreateCommunityUseCase::new(repo);
//
//         let input = CreateCommunity {
//             slug: "Community-Test".to_string(),
//             name: "".to_string(),
//             is_public: false,
//         };
//
//         let result = use_case.execute(input, valid_auth_context());
//
//         assert!(matches!(result, Err(ApplicationError::InvalidName)));
//     }
//
//     #[test]
//     fn fails_when_slug_is_invalid() {
//         let repo = Arc::new(FakeCommunityRepository::success());
//
//         let use_case = CreateCommunityUseCase::new(repo);
//
//         let input = CreateCommunity {
//             slug: "".to_string(),
//             name: "Community Test".to_string(),
//             is_public: false,
//         };
//
//         let result = use_case.execute(input, valid_auth_context());
//
//         assert!(matches!(result, Err(ApplicationError::InvalidSlug)));
//     }
//
//     #[test]
//     fn fails_when_repository_fails() {
//         let repo = Arc::new(FakeCommunityRepository::fail());
//
//         let use_case = CreateCommunityUseCase::new(repo);
//
//         let result = use_case.execute(valid_input(),valid_auth_context());
//
//         assert!(matches!(
//             result,
//             Err(ApplicationError::Common(
//                 CommonApplicationError::Infrastructure
//             ))
//         ));
//     }
//
//     #[test]
//     fn fails_when_slug_already_exists() {
//         let repo = Arc::new(FakeCommunityRepository::with_existing_slug("community-test"));
//
//         let use_case = CreateCommunityUseCase::new(repo);
//
//         let result = use_case.execute(valid_input(),valid_auth_context());
//
//         assert!(matches!(result, Err(ApplicationError::SlugAlreadyExists)));
//     }
// }
use crate::{
    application::{
        commands::create_community::CreateCommunity,
        errors::community_creation::CommunityCreationError,
        ports::{
            inbound::community_creation::CommunityCreationPort,
            outbound::community_repository::CommunityRepositoryPort,
        },
        results::community_created::CommunityCreated,
    },
    domain::{
        aggregates::community::Community,
        value_objects::{
            community_id::CommunityId, community_name::CommunityName, community_slug::CommunitySlug,
        },
    },
};
use iam::domain::value_objects::AccountId;
use shared::{application::auth_context::AuthContext, error::SystemError};
use std::sync::Arc;

pub struct CreateCommunityUseCase {
    community_repository: Arc<dyn CommunityRepositoryPort>,
}

impl CreateCommunityUseCase {
    pub fn new(community_repository: Arc<dyn CommunityRepositoryPort>) -> Self {
        Self {
            community_repository,
        }
    }
}

impl CommunityCreationPort for CreateCommunityUseCase {
    fn execute(
        &self,
        data: CreateCommunity,
        auth: AuthContext,
    ) -> Result<CommunityCreated, SystemError> {
        let slug = CommunitySlug::new(data.slug)?;
        let existing_slug = self
            .community_repository
            .find_by_slug(slug.clone().as_str());
        if existing_slug.is_some() {
            return Err(CommunityCreationError::SlugAlreadyExists.into());
        }
        let account_id = AccountId::from_str(auth.account_id.as_str())?;
        let id = CommunityId::generate();
        let name = CommunityName::new(data.name.clone())?;

        let community = Community::create(id, account_id, slug, name, data.is_public);

        self.community_repository.save(&community)?;

        Ok(CommunityCreated {
            id: community.id().as_uuid().to_string(),
            name: community.name().as_str().to_string(),
            slug: community.slug().as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            commands::create_community::CreateCommunity,
            errors::error_codes::{COMMUNITIES_REPOSITORY_ERROR, COMMUNITIES_SLUG_ALREADY_EXISTS},
            ports::{
                inbound::community_creation::CommunityCreationPort,
                outbound::community_repository::test_utils::FakeCommunityRepository,
            },
            use_cases::create_community::CreateCommunityUseCase,
        },
        domain::errors::error_codes::{
            COMMUNITIES_INVALID_COMMUNITY_NAME, COMMUNITIES_INVALID_COMMUNITY_SLUG,
        },
    };
    use iam::domain::value_objects::AccountId;
    use shared::application::auth_context::AuthContext;
    use std::sync::Arc;

    fn valid_auth_context() -> AuthContext {
        AuthContext {
            account_id: AccountId::generate().as_uuid().to_string(),
        }
    }

    fn valid_input() -> CreateCommunity {
        CreateCommunity {
            slug: "Community-Test".to_string(),
            name: "Community Test".to_string(),
            is_public: true,
        }
    }

    #[test]
    fn create_community_successfully() {
        let repo = Arc::new(FakeCommunityRepository::success());

        let use_case = CreateCommunityUseCase::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_name_is_invalid() {
        let repo = Arc::new(FakeCommunityRepository::success());

        let use_case = CreateCommunityUseCase::new(repo);

        let input = CreateCommunity {
            slug: "Community-Test".to_string(),
            name: "".to_string(),
            is_public: false,
        };

        let result = use_case.execute(input, valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_INVALID_COMMUNITY_NAME);
    }

    #[test]
    fn fails_when_slug_is_invalid() {
        let repo = Arc::new(FakeCommunityRepository::success());

        let use_case = CreateCommunityUseCase::new(repo);

        let input = CreateCommunity {
            slug: "".to_string(),
            name: "Community Test".to_string(),
            is_public: false,
        };

        let result = use_case.execute(input, valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_INVALID_COMMUNITY_SLUG);
    }

    #[test]
    fn fails_when_repository_fails() {
        let repo = Arc::new(FakeCommunityRepository::fail());

        let use_case = CreateCommunityUseCase::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_REPOSITORY_ERROR);
    }

    #[test]
    fn fails_when_slug_already_exists() {
        let repo = Arc::new(FakeCommunityRepository::with_existing_slug(
            "community-test",
        ));

        let use_case = CreateCommunityUseCase::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_SLUG_ALREADY_EXISTS);
    }
}

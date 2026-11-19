use crate::{
    application::{
        commands::create_community::{
            command::CreateCommunityCommand, response::CreateCommunityResponse,
        },
        errors::community_creation::CommunityCreationError,
        ports::{
            inbound::create_community_use_case::CreateCommunityUseCase,
            outbound::community_repository::CommunityRepositoryPort,
        },
    },
    domain::model::community::{
        community::Community, community_id::CommunityId, community_name::CommunityName,
        community_slug::CommunitySlug,
    },
};
use iam::domain::model::account::account_id::AccountId;
use shared::{domain::model::verified_identity::VerifiedIdentity, error::SystemError};
use std::sync::Arc;

pub struct CreateCommunity {
    community_repository: Arc<dyn CommunityRepositoryPort>,
}

impl CreateCommunity {
    pub fn new(community_repository: Arc<dyn CommunityRepositoryPort>) -> Self {
        Self {
            community_repository,
        }
    }
}

impl CreateCommunityUseCase for CreateCommunity {
    fn execute(
        &self,
        data: CreateCommunityCommand,
        auth: VerifiedIdentity,
    ) -> Result<CreateCommunityResponse, SystemError> {
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

        Ok(CreateCommunityResponse {
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
            commands::create_community::{
                command::CreateCommunityCommand, create_community::CreateCommunity,
            },
            errors::error_codes::{COMMUNITIES_REPOSITORY_ERROR, COMMUNITIES_SLUG_ALREADY_EXISTS},
            ports::{
                inbound::create_community_use_case::CreateCommunityUseCase,
                outbound::community_repository::test_utils::FakeCommunityRepository,
            },
        },
        domain::errors::error_codes::{
            COMMUNITIES_INVALID_COMMUNITY_NAME, COMMUNITIES_INVALID_COMMUNITY_SLUG,
        },
    };
    use iam::domain::model::account::account_id::AccountId;
    use shared::domain::model::verified_identity::VerifiedIdentity;
    use std::sync::Arc;

    fn valid_auth_context() -> VerifiedIdentity {
        VerifiedIdentity {
            account_id: AccountId::generate().as_uuid().to_string(),
        }
    }

    fn valid_input() -> CreateCommunityCommand {
        CreateCommunityCommand {
            slug: "Community-Test".to_string(),
            name: "Community Test".to_string(),
            is_public: true,
        }
    }

    #[test]
    fn create_community_successfully() {
        let repo = Arc::new(FakeCommunityRepository::success());

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_name_is_invalid() {
        let repo = Arc::new(FakeCommunityRepository::success());

        let use_case = CreateCommunity::new(repo);

        let input = CreateCommunityCommand {
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

        let use_case = CreateCommunity::new(repo);

        let input = CreateCommunityCommand {
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

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_REPOSITORY_ERROR);
    }

    #[test]
    fn fails_when_slug_already_exists() {
        let repo = Arc::new(FakeCommunityRepository::with_existing_slug(
            "community-test",
        ));

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_SLUG_ALREADY_EXISTS);
    }
}

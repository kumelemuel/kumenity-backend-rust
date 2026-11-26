use crate::{
    application::{
        commands::create_community::{
            command::CreateCommunityCommand, response::CreateCommunityResponse,
        },
        errors::community_creation::CommunityCreationError,
        ports::{
            inbound::create_community_use_case::CreateCommunityUseCase,
            outbound::community_repository::CommunityRepository,
        },
    },
    domain::model::community::{
        community::Community, community_id::CommunityId, community_name::CommunityName,
        community_slug::CommunitySlug, owner_id::OwnerId,
    },
};
use shared::{domain::model::verified_identity::VerifiedIdentity, error::SystemError};
use std::sync::Arc;

pub struct CreateCommunity {
    community_repository: Arc<dyn CommunityRepository>,
}

impl CreateCommunity {
    pub fn new(community_repository: Arc<dyn CommunityRepository>) -> Self {
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
            .find_by_slug(slug.clone().as_str())
            .map_err(SystemError::Unexpected)?;
        if existing_slug.is_some() {
            return Err(CommunityCreationError::SlugAlreadyExists.into());
        }
        let owner_id = OwnerId::from_str(auth.account_id.as_str())?;
        let id = CommunityId::generate();
        let name = CommunityName::new(data.name.clone())?;

        let community = Community::create(id, owner_id, slug, name, data.is_public);

        self.community_repository
            .save(&community)
            .map_err(SystemError::Unexpected)?;

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
            errors::error_codes::COMMUNITIES_SLUG_ALREADY_EXISTS,
            ports::inbound::create_community_use_case::CreateCommunityUseCase,
            testing::{CommunityOperation, FakeCommunityAdapter},
        },
        domain::errors::error_codes::{
            COMMUNITIES_INVALID_COMMUNITY_NAME, COMMUNITIES_INVALID_COMMUNITY_SLUG,
        },
    };
    use shared::{domain::model::verified_identity::VerifiedIdentity, error::SystemError};
    use std::sync::Arc;

    fn valid_auth_context() -> VerifiedIdentity {
        VerifiedIdentity {
            account_id: "6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60".to_string(),
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
        let repo = Arc::new(FakeCommunityAdapter::empty());

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        assert!(result.is_ok());
    }

    #[test]
    fn fails_when_name_is_invalid() {
        let repo = Arc::new(FakeCommunityAdapter::empty());

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
        let repo = Arc::new(FakeCommunityAdapter::empty());

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
    fn fails_with_unexpected_when_repository_fails() {
        let repo = Arc::new(FakeCommunityAdapter::failing());

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(FakeCommunityAdapter::empty().failing_on(CommunityOperation::Save));

        let use_case = CreateCommunity::new(repo);

        let result = use_case.execute(valid_input(), valid_auth_context());

        assert!(
            matches!(result, Err(SystemError::Unexpected(_))),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_slug_already_exists() {
        let repo = Arc::new(FakeCommunityAdapter::empty());

        let use_case = CreateCommunity::new(repo);

        use_case
            .execute(valid_input(), valid_auth_context())
            .expect("the first community with this slug is created");

        let result = use_case.execute(valid_input(), valid_auth_context());

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), COMMUNITIES_SLUG_ALREADY_EXISTS);
    }
}

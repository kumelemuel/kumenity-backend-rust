use super::{CreateCommunityCommand, CreateCommunityError, CreateCommunityResponse};
use crate::{
    application::{
        errors::{conflict::Conflict, invalid_owner::InvalidOwner, validation::Validation},
        ports::{
            inbound::create_community_use_case::CreateCommunityUseCase,
            outbound::community_repository::CommunityRepository,
        },
    },
    domain::{
        errors::{
            invalid_community_name::InvalidCommunityName,
            invalid_community_slug::InvalidCommunitySlug, invalid_owner_id::InvalidOwnerId,
        },
        model::community::{
            community::Community, community_id::CommunityId, community_name::CommunityName,
            community_slug::CommunitySlug, owner_id::OwnerId,
        },
    },
};
use shared::Failure;
use std::sync::Arc;

/// Orchestrates (HEX-7): builds the value objects, checks the slug for a
/// conflict, creates the community, saves it and maps it.
///
/// The order is fixed by `app/tests/`: the slug, then its conflict
/// (`create_checks_slug_uniqueness_before_subject_format`), then the owner,
/// then the name.
pub struct CreateCommunityHandler {
    community_repository: Arc<dyn CommunityRepository>,
}

impl CreateCommunityHandler {
    pub fn new(community_repository: Arc<dyn CommunityRepository>) -> Self {
        Self {
            community_repository,
        }
    }
}

impl CreateCommunityUseCase for CreateCommunityHandler {
    fn execute(
        &self,
        command: CreateCommunityCommand,
    ) -> Result<CreateCommunityResponse, Failure<CreateCommunityError>> {
        // Domain errors → flat leaf of the contract, by variant (ERR-2).
        let slug = CommunitySlug::new(command.slug)
            .map_err(|InvalidCommunitySlug| Failure::expected(Validation { field: "slug" }))?;

        // `find_by_slug` answers an Option; this use case decides that
        // existing is a conflict (ERR-1). `?` propagates the `Fault` as
        // tree B.
        if self
            .community_repository
            .find_by_slug(slug.as_str())?
            .is_some()
        {
            return Err(Failure::expected(Conflict { resource: "slug" }));
        }

        let owner_id = OwnerId::from_str(&command.owner_id).map_err(|error| {
            Failure::expected(match error {
                InvalidOwnerId::Nil => InvalidOwner::Nil,
                InvalidOwnerId::WrongFormat => InvalidOwner::WrongFormat,
            })
        })?;
        let name = CommunityName::new(command.name)
            .map_err(|InvalidCommunityName| Failure::expected(Validation { field: "name" }))?;

        let community = Community::create(
            CommunityId::generate(),
            owner_id,
            slug,
            name,
            command.is_public,
        );

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
    use super::*;
    use crate::application::testing::{CommunityOperation, FakeCommunityAdapter};

    type Outcome = Result<CreateCommunityResponse, Failure<CreateCommunityError>>;

    const OWNER: &str = "6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60";

    fn valid_input() -> CreateCommunityCommand {
        with(OWNER, "Community-Test", "Community Test")
    }

    fn with(owner_id: &str, slug: &str, name: &str) -> CreateCommunityCommand {
        CreateCommunityCommand {
            owner_id: owner_id.to_string(),
            slug: slug.to_string(),
            name: name.to_string(),
            is_public: true,
        }
    }

    fn create_with(repo: Arc<FakeCommunityAdapter>, input: CreateCommunityCommand) -> Outcome {
        CreateCommunityHandler::new(repo).execute(input)
    }

    #[test]
    fn create_community_successfully() {
        let result = create_with(Arc::new(FakeCommunityAdapter::empty()), valid_input());

        let response = result.expect("the community is created");
        assert_eq!(response.slug, "community-test");
        assert_eq!(response.name, "Community Test");
    }

    #[test]
    fn fails_with_validation_on_invalid_slug() {
        let result = create_with(
            Arc::new(FakeCommunityAdapter::empty()),
            with(OWNER, "", "Community Test"),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::Validation(
                    Validation { field: "slug" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_validation_on_invalid_name() {
        let result = create_with(
            Arc::new(FakeCommunityAdapter::empty()),
            with(OWNER, "Community-Test", ""),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::Validation(
                    Validation { field: "name" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_conflict_on_taken_slug() {
        let repo = Arc::new(FakeCommunityAdapter::empty());
        create_with(repo.clone(), valid_input())
            .expect("the first community with this slug is created");

        let result = create_with(repo, valid_input());

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::Conflict(
                    Conflict { resource: "slug" }
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_owner_id_is_invalid() {
        let result = create_with(
            Arc::new(FakeCommunityAdapter::empty()),
            with("not-a-uuid", "Community-Test", "Community Test"),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::InvalidOwner(
                    InvalidOwner::WrongFormat
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_when_owner_id_is_nil() {
        let result = create_with(
            Arc::new(FakeCommunityAdapter::empty()),
            with(
                "00000000-0000-0000-0000-000000000000",
                "Community-Test",
                "Community Test",
            ),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::InvalidOwner(
                    InvalidOwner::Nil
                )))
            ),
            "{result:?}"
        );
    }

    /// Slug, then conflict, then owner: as
    /// `create_checks_slug_uniqueness_before_subject_format` fixes.
    #[test]
    fn fails_with_conflict_on_taken_slug_before_checking_the_owner() {
        let repo = Arc::new(FakeCommunityAdapter::empty());
        create_with(repo.clone(), valid_input())
            .expect("the first community with this slug is created");

        let result = create_with(repo, with("not-a-uuid", "Community-Test", "Community Test"));

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::Conflict(
                    Conflict { resource: "slug" }
                )))
            ),
            "{result:?}"
        );
    }

    /// The owner, then the name: the order of today.
    #[test]
    fn fails_when_owner_id_is_invalid_before_validating_the_name() {
        let result = create_with(
            Arc::new(FakeCommunityAdapter::empty()),
            with("not-a-uuid", "Community-Test", ""),
        );

        assert!(
            matches!(
                result,
                Err(Failure::Expected(CreateCommunityError::InvalidOwner(
                    InvalidOwner::WrongFormat
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unexpected_when_find_by_slug_fails() {
        let repo =
            Arc::new(FakeCommunityAdapter::empty().failing_on(CommunityOperation::FindBySlug));

        let result = create_with(repo, valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }

    #[test]
    fn fails_with_unexpected_when_save_fails() {
        let repo = Arc::new(FakeCommunityAdapter::empty().failing_on(CommunityOperation::Save));

        let result = create_with(repo, valid_input());

        assert!(matches!(result, Err(Failure::Unexpected(_))), "{result:?}");
    }
}

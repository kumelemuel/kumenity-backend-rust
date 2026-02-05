use std::sync::Arc;
use iam::domain::value_objects::AccountId;
use shared::application::auth_context::AuthContext;
use shared::application::common_application_error::CommonApplicationError;
use crate::application::commands::create_community::CreateCommunity;
use crate::application::errors::application_error::ApplicationError;
use crate::application::ports::inbound::community_creation::CommunityCreationPort;
use crate::application::ports::outbound::community_repository::CommunityRepositoryPort;
use crate::application::results::community_created::CommunityCreated;
use crate::domain::aggregates::community::Community;
use crate::domain::value_objects::community_id::CommunityId;
use crate::domain::value_objects::community_name::CommunityName;

pub struct CreateCommunityUseCase {
    community_repository: Arc<dyn CommunityRepositoryPort>,
}

impl CreateCommunityUseCase {
    pub fn new(community_repository: Arc<dyn CommunityRepositoryPort>) -> Self {
        Self { community_repository }
    }
}

impl CommunityCreationPort for CreateCommunityUseCase {
    fn execute(&self, data: CreateCommunity, auth: AuthContext) -> Result<CommunityCreated, ApplicationError> {

        let account_id = AccountId::from_str(auth.subject.as_str());
        let id = CommunityId::generate();
        let name = CommunityName::new(data.name).map_err(|_| ApplicationError::InvalidName);

        let community = Community::create(id, account_id.unwrap(), name?, data.is_public, None);

        self.community_repository.save(&community).map_err(|_| CommonApplicationError::Infrastructure)?;

        Ok( CommunityCreated {
            id: community.id().as_uuid().to_string(),
            name: community.name().as_str().to_string(),
        })

    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use iam::domain::value_objects::AccountId;
    use shared::application::auth_context::AuthContext;
    use shared::application::common_application_error::CommonApplicationError;
    use crate::application::commands::create_community::CreateCommunity;
    use crate::application::errors::application_error::ApplicationError;
    use crate::application::ports::inbound::community_creation::CommunityCreationPort;
    use crate::application::ports::outbound::community_repository::test_utils::FakeCommunityRepository;
    use crate::application::use_cases::create_community::CreateCommunityUseCase;

    fn valid_auth_context() -> AuthContext {
        AuthContext {
            subject: AccountId::generate().as_uuid().to_string(),
        }
    }

    fn valid_input() -> CreateCommunity {
        CreateCommunity {
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
            name: "".to_string(),
            is_public: false,
        };

        let result = use_case.execute(input, valid_auth_context());

        assert!(matches!(result, Err(ApplicationError::InvalidName)));
    }

    #[test]
    fn fails_when_repository_fails() {
        let repo = Arc::new(FakeCommunityRepository::fail());

        let use_case = CreateCommunityUseCase::new(repo);

        let result = use_case.execute(valid_input(),valid_auth_context());

        assert!(matches!(
            result,
            Err(ApplicationError::Common(
                CommonApplicationError::Infrastructure
            ))
        ));
    }
}
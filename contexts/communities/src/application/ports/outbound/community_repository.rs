use crate::{
    application::errors::community_repository::CommunityRepositoryError,
    domain::model::community::community::Community,
};

pub trait CommunityRepositoryPort: Send + Sync {
    fn find_by_slug(&self, slug: &str) -> Option<Community>;
    fn save(&self, community: &Community) -> Result<(), CommunityRepositoryError>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::{
        application::{
            errors::community_repository::CommunityRepositoryError,
            ports::outbound::community_repository::CommunityRepositoryPort,
        },
        domain::model::community::community::Community,
    };

    pub struct FakeCommunityRepository {
        should_fail: bool,
        existing_slug: Option<String>,
    }

    impl FakeCommunityRepository {
        pub fn success() -> Self {
            Self {
                should_fail: false,
                existing_slug: None,
            }
        }

        pub fn fail() -> Self {
            Self {
                should_fail: true,
                existing_slug: None,
            }
        }

        pub fn with_existing_slug(slug: &str) -> Self {
            Self {
                should_fail: false,
                existing_slug: Some(slug.to_string()),
            }
        }
    }

    impl CommunityRepositoryPort for FakeCommunityRepository {
        fn find_by_slug(&self, slug: &str) -> Option<Community> {
            self.existing_slug
                .as_ref()
                .filter(|c| c.as_str() == slug)
                .map(|_| Community::dummy_community())
        }

        fn save(&self, _community: &Community) -> Result<(), CommunityRepositoryError> {
            if self.should_fail {
                Err(CommunityRepositoryError("Unexpected error".to_string()))
            } else {
                Ok(())
            }
        }
    }
}

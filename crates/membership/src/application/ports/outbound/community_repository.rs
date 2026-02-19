use crate::domain::aggregates::community::Community;

pub trait CommunityRepositoryPort: Send + Sync {
    fn find_by_id(&self, id: &str) -> Option<Community>;

    fn find_by_slug(&self, slug: &str) -> Option<Community>;
    fn save(&self, community: &Community) -> Result<(), String>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::community_repository::CommunityRepositoryPort;
    use crate::domain::aggregates::community::Community;

    pub struct FakeCommunityRepository {
        should_fail: bool,
        activated: bool,
        existing_id: Option<String>,
        existing_slug: Option<String>,
    }

    impl FakeCommunityRepository {
        pub fn success() -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_id: None,
                existing_slug: None,
            }
        }

        pub fn fail() -> Self {
            Self {
                should_fail: true,
                activated: false,
                existing_id: None,
                existing_slug: None,
            }
        }

        pub fn with_existing_slug(slug: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_id: None,
                existing_slug: Some(slug.to_string()),
            }
        }

        pub fn with_existing_id(id: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_id: Some(id.to_string()),
                existing_slug: None,
            }
        }

        pub fn active_with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                activated: true,
                existing_id: Some(username.to_string()),
                existing_slug: None,
            }
        }
    }

    impl CommunityRepositoryPort for FakeCommunityRepository {
        fn find_by_id(&self, id: &str) -> Option<Community> {
            self.existing_id
                .as_ref()
                .filter(|c| c.as_str() == id)
                .map(|_| {
                    Community::dummy_community()
                })
        }

        fn find_by_slug(&self, slug: &str) -> Option<Community> {
            self.existing_slug
                .as_ref()
                .filter(|c| c.as_str() == slug)
                .map(|_| {
                    Community::dummy_community()
                })
        }

        fn save(&self, _community: &Community) -> Result<(), String> {
            if self.should_fail {
                Err("Unexpected error".to_string())
            } else {
                Ok(())
            }
        }
    }
}
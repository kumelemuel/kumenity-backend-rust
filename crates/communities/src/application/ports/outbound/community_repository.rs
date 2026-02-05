use crate::domain::aggregates::community::Community;

pub trait CommunityRepositoryPort: Send + Sync {
    fn find_by_id(&self, id: &str) -> Option<Community>;
    fn save(&self, community: &Community) -> Result<(), String>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::community_repository::CommunityRepositoryPort;
    use crate::domain::aggregates::community::Community;

    pub struct FakeCommunityRepository {
        should_fail: bool,
        activated: bool,
        existing_username: Option<String>,
        existing_email: Option<String>,
    }

    impl FakeCommunityRepository {
        pub fn success() -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: None,
                existing_email: None,
            }
        }

        pub fn fail() -> Self {
            Self {
                should_fail: true,
                activated: false,
                existing_username: None,
                existing_email: None,
            }
        }

        pub fn with_existing_email(email: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: None,
                existing_email: Some(email.to_string()),
            }
        }

        pub fn with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: Some(username.to_string()),
                existing_email: None,
            }
        }

        pub fn active_with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                activated: true,
                existing_username: Some(username.to_string()),
                existing_email: None,
            }
        }
    }

    impl CommunityRepositoryPort for FakeCommunityRepository {
        fn find_by_id(&self, id: &str) -> Option<Community> {
            self.existing_username
                .as_ref()
                .filter(|c| c.as_str() == id)
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
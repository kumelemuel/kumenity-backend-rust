use crate::domain::model::community::community::Community;
use shared::Fault;

/// Persistencia de `Community`. No declara errores (ERR-3): cualquier falla
/// técnica es `Fault` (árbol B).
pub trait CommunityRepository: Send + Sync {
    /// "No existe" no es un error: `None` (ERR-1).
    fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault>;
    fn save(&self, community: &Community) -> Result<(), Fault>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::{
        application::ports::outbound::community_repository::CommunityRepository,
        domain::model::community::community::Community,
    };
    use shared::Fault;

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

    impl CommunityRepository for FakeCommunityRepository {
        fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault> {
            Ok(self
                .existing_slug
                .as_ref()
                .filter(|c| c.as_str() == slug)
                .map(|_| Community::dummy_community()))
        }

        fn save(&self, _community: &Community) -> Result<(), Fault> {
            if self.should_fail {
                Err(Fault::new("Unexpected error"))
            } else {
                Ok(())
            }
        }
    }
}

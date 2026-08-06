use super::error_codes::COMMUNITIES_INVALID_COMMUNITY_ID;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCommunityId;

impl fmt::Display for InvalidCommunityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community id")
    }
}

impl std::error::Error for InvalidCommunityId {}

impl ErrorLayout for InvalidCommunityId {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        COMMUNITIES_INVALID_COMMUNITY_ID
    }

    fn message(&self) -> &'static str {
        "The community ID is invalid."
    }
}

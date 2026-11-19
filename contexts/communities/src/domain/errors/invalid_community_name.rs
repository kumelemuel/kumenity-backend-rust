use super::error_codes::COMMUNITIES_INVALID_COMMUNITY_NAME;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunityName;

impl fmt::Display for InvalidCommunityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community name")
    }
}

impl std::error::Error for InvalidCommunityName {}

impl ErrorLayout for InvalidCommunityName {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        COMMUNITIES_INVALID_COMMUNITY_NAME
    }

    fn message(&self) -> &'static str {
        "Please enter a valid community name."
    }
}

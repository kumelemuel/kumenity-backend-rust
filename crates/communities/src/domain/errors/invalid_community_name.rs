use super::error_codes::COMMUNITIES_INVALID_COMMUNITY_NAME;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunityName;

impl fmt::Display for InvalidCommunityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community name")
    }
}

impl std::error::Error for InvalidCommunityName {}

impl LayerError for InvalidCommunityName {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        COMMUNITIES_INVALID_COMMUNITY_NAME
    }

    fn message(&self) -> &'static str {
        "Please enter a valid community name."
    }
}

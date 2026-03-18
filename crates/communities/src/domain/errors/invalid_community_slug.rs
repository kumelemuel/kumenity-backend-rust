use super::error_codes::COMMUNITIES_INVALID_COMMUNITY_SLUG;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunitySlug;

impl fmt::Display for InvalidCommunitySlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community slug")
    }
}

impl std::error::Error for InvalidCommunitySlug {}

impl LayerError for InvalidCommunitySlug {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        COMMUNITIES_INVALID_COMMUNITY_SLUG
    }

    fn message(&self) -> &'static str {
        "Please enter a valid community slug."
    }
}

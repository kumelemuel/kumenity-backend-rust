use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunitySlug;

impl fmt::Display for InvalidCommunitySlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community slug")
    }
}

impl DomainError for InvalidCommunitySlug {
    fn code(&self) -> &'static str {
        "COMMUNITIES_INVALID_COMMUNITY_SLUG"
    }
}

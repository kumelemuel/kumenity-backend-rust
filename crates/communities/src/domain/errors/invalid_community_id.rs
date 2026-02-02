use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCommunityId;

impl fmt::Display for InvalidCommunityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community id")
    }
}

impl DomainError for InvalidCommunityId {
    fn code(&self) -> &'static str {
        "COMMUNITIES_INVALID_COMMUNITY_ID"
    }
}

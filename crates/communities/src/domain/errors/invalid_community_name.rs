use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunityName;

impl fmt::Display for InvalidCommunityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community name")
    }
}

impl DomainError for InvalidCommunityName {
    fn code(&self) -> &'static str {
        "COMMUNITIES_INVALID_COMMUNITY_NAME"
    }
}

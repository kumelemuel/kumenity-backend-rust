use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidMembershipStatusTransition;

impl fmt::Display for InvalidMembershipStatusTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid membership status transition")
    }
}

impl DomainError for InvalidMembershipStatusTransition {
    fn code(&self) -> &'static str {
        "COMMUNITIES_INVALID_MEMBERSHIP_STATUS_TRANSITION"
    }
}

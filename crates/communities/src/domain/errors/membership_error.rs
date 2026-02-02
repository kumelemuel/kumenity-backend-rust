use std::fmt;
use shared::domain::DomainError;

#[derive(Debug, PartialEq, Eq)]
pub enum MembershipError {
    CannotChangeOwnerRole,
    InvalidStatusTransition,
    InactiveMembership,
}

impl fmt::Display for MembershipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            MembershipError::CannotChangeOwnerRole => { write!(f, "cannot change owner role") },
            MembershipError::InvalidStatusTransition => { write!(f, "invalid status transition") },
            MembershipError::InactiveMembership => { write!(f, "inactive membership") },
        }
    }
}

impl DomainError for MembershipError {
    fn code(&self) -> &'static str {
        match self {
            MembershipError::CannotChangeOwnerRole => "COMMUNITIES_MEMBER_CHANGE_OWNER_ROLE",
            MembershipError::InvalidStatusTransition => "COMMUNITIES_MEMBERS_INVALID_STATUS_TRANSITION",
            MembershipError::InactiveMembership => "COMMUNITIES_MEMBER_INACTIVE",
        }
    }
}

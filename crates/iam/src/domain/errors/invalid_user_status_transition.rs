use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidUserStatusTransition;

impl fmt::Display for InvalidUserStatusTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid user status transition")
    }
}

impl DomainError for InvalidUserStatusTransition {
    fn code(&self) -> &'static str {
        "IAM_INVALID_USER_STATUS_TRANSITION"
    }
}

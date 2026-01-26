use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidAccountStatusTransition;

impl fmt::Display for InvalidAccountStatusTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid account status transition")
    }
}

impl DomainError for InvalidAccountStatusTransition {
    fn code(&self) -> &'static str {
        "IAM_INVALID_ACCOUNT_STATUS_TRANSITION"
    }
}

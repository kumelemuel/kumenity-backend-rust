use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidUserId;

impl fmt::Display for InvalidUserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid user id")
    }
}

impl DomainError for InvalidUserId {
    fn code(&self) -> &'static str {
        "IAM_INVALID_USER_ID"
    }
}

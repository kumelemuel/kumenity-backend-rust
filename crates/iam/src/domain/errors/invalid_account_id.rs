use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidAccountId;

impl fmt::Display for InvalidAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid account id")
    }
}

impl DomainError for InvalidAccountId {
    fn code(&self) -> &'static str {
        "IAM_INVALID_ACCOUNT_ID"
    }
}

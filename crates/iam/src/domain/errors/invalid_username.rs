use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidUsername;

impl fmt::Display for InvalidUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid username")
    }
}

impl DomainError for InvalidUsername {
    fn code(&self) -> &'static str {
        "IAM_INVALID_USERNAME"
    }
}

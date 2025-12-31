use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidHashedPassword;

impl fmt::Display for InvalidHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid hashed password")
    }
}

impl DomainError for InvalidHashedPassword {
    fn code(&self) -> &'static str {
        "IAM_INVALID_HASHED_PASSWORD"
    }
}

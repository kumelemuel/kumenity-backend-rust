use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidEmail;

impl fmt::Display for InvalidEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid email")
    }
}

impl DomainError for InvalidEmail {
    fn code(&self) -> &'static str {
        "IAM_INVALID_EMAIL"
    }
}

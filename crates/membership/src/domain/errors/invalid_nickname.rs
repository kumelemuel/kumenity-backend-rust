use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidNickname;

impl fmt::Display for InvalidNickname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid nickname")
    }
}

impl DomainError for InvalidNickname {
    fn code(&self) -> &'static str {
        "COMMUNITIES_INVALID_NICKNAME"
    }
}

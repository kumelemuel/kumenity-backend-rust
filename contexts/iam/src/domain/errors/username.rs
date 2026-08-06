use super::error_codes::IAM_INVALID_USERNAME;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum UsernameError {
    Invalid,
}

impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsernameError::Invalid => write!(f, "Please enter a valid username"),
        }
    }
}

impl std::error::Error for UsernameError {}

impl ErrorLayout for UsernameError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            UsernameError::Invalid => IAM_INVALID_USERNAME,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            UsernameError::Invalid => "Please enter a valid username.",
        }
    }
}

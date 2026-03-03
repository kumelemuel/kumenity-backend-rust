use super::error_codes::IAM_INVALID_USERNAME;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum UsernameError {
    Invalid,
}

impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsernameError::Invalid => write!(f, "Invalid username"),
        }
    }
}

impl std::error::Error for UsernameError {}

impl LayerError for UsernameError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            UsernameError::Invalid => IAM_INVALID_USERNAME,
        }
    }
}

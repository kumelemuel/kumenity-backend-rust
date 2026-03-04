use super::error_codes::IAM_INVALID_EMAIL;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Invalid,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::Invalid => write!(f, "Please enter a valid email address"),
        }
    }
}

impl std::error::Error for EmailError {}

impl LayerError for EmailError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            EmailError::Invalid => IAM_INVALID_EMAIL,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            EmailError::Invalid => "Please enter a valid email address.",
        }
    }
}

use super::error_codes::IAM_INVALID_HASHED_PASSWORD;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum HashedPasswordError {
    Invalid,
}

impl fmt::Display for HashedPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashedPasswordError::Invalid => write!(f, "Invalid hashed password"),
        }
    }
}

impl std::error::Error for HashedPasswordError {}

impl LayerError for HashedPasswordError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            HashedPasswordError::Invalid => IAM_INVALID_HASHED_PASSWORD,
        }
    }
}

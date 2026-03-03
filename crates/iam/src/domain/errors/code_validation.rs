use super::error_codes::IAM_INVALID_CODE_VALIDATION;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeValidationError {
    Invalid,
}

impl fmt::Display for CodeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeValidationError::Invalid => write!(f, "Invalid code validation"),
        }
    }
}

impl std::error::Error for CodeValidationError {}

impl LayerError for CodeValidationError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            CodeValidationError::Invalid => IAM_INVALID_CODE_VALIDATION,
        }
    }
}

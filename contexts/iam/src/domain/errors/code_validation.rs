use super::error_codes::IAM_INVALID_CODE_VALIDATION;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeValidationError {
    Invalid,
}

impl fmt::Display for CodeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeValidationError::Invalid => write!(f, "The verification code is invalid"),
        }
    }
}

impl std::error::Error for CodeValidationError {}

impl ErrorLayout for CodeValidationError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            CodeValidationError::Invalid => IAM_INVALID_CODE_VALIDATION,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            CodeValidationError::Invalid => "The verification code is invalid.",
        }
    }
}

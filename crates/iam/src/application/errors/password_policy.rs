use super::error_codes::IAM_PASSWORD_TOO_SHORT;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub enum PasswordPolicyError {
    TooShort,
}

impl fmt::Display for PasswordPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordPolicyError::TooShort => write!(f, "Invalid account ID"),
        }
    }
}

impl std::error::Error for PasswordPolicyError {}

impl LayerError for PasswordPolicyError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Application
    }

    fn code(&self) -> &'static str {
        match self {
            PasswordPolicyError::TooShort => IAM_PASSWORD_TOO_SHORT,
        }
    }
}

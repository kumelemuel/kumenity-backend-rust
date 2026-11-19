use crate::domain::errors::error_codes::IAM_PASSWORD_TOO_SHORT;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordPolicyError {
    TooShort,
}

impl fmt::Display for PasswordPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordPolicyError::TooShort => {
                write!(f, "Password must be at least 8 characters long")
            }
        }
    }
}

impl std::error::Error for PasswordPolicyError {}

impl ErrorLayout for PasswordPolicyError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Application
    }

    fn code(&self) -> &'static str {
        match self {
            PasswordPolicyError::TooShort => IAM_PASSWORD_TOO_SHORT,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            PasswordPolicyError::TooShort => "Password must be at least 8 characters long.",
        }
    }
}

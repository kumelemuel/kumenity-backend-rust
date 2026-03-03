use super::error_codes::IAM_TOKEN_GENERATOR_ERROR;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub struct TokenGeneratorError(pub String);

impl fmt::Display for TokenGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "We couldn't create your session right now")
    }
}

impl std::error::Error for TokenGeneratorError {}

impl LayerError for TokenGeneratorError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Application
    }

    fn code(&self) -> &'static str {
        IAM_TOKEN_GENERATOR_ERROR
    }

    fn message(&self) -> &'static str {
        "We couldn't create your session right now. Please try again."
    }
}

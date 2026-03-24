use super::error_codes::COMMUNITIES_REPOSITORY_ERROR;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub struct CommunityRepositoryError(pub String);

impl fmt::Display for CommunityRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "We couldn't complete your request right now")
    }
}

impl std::error::Error for CommunityRepositoryError {}

impl LayerError for CommunityRepositoryError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Application
    }

    fn code(&self) -> &'static str {
        COMMUNITIES_REPOSITORY_ERROR
    }

    fn message(&self) -> &'static str {
        "We couldn't complete your request right now. Please try again."
    }
}

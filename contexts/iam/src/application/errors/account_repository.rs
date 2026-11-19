use super::error_codes::IAM_ACCOUNT_REPOSITORY_ERROR;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug)]
pub struct AccountRepositoryError(pub String);

impl fmt::Display for AccountRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AccountRepositoryError {}

impl ErrorLayout for AccountRepositoryError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Application
    }

    fn code(&self) -> &'static str {
        IAM_ACCOUNT_REPOSITORY_ERROR
    }

    fn message(&self) -> &'static str {
        "We couldn't complete your request right now. Please try again."
    }
}

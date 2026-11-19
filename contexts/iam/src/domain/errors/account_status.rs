use super::error_codes::IAM_INVALID_ACCOUNT_STATUS_TRANSITION;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum AccountStatusError {
    InvalidTransition,
}

impl fmt::Display for AccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountStatusError::InvalidTransition => {
                write!(f, "This account status change is not allowed")
            }
        }
    }
}

impl std::error::Error for AccountStatusError {}

impl ErrorLayout for AccountStatusError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountStatusError::InvalidTransition => IAM_INVALID_ACCOUNT_STATUS_TRANSITION,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AccountStatusError::InvalidTransition => "This account status change is not allowed.",
        }
    }
}

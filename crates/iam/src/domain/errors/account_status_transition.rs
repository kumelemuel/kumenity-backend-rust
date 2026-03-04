use super::error_codes::IAM_INVALID_ACCOUNT_STATUS_TRANSITION;
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum AccountStatusTransitionError {
    Invalid,
}

impl fmt::Display for AccountStatusTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountStatusTransitionError::Invalid => {
                write!(f, "This account status change is not allowed")
            }
        }
    }
}

impl std::error::Error for AccountStatusTransitionError {}

impl LayerError for AccountStatusTransitionError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountStatusTransitionError::Invalid => IAM_INVALID_ACCOUNT_STATUS_TRANSITION,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AccountStatusTransitionError::Invalid => "This account status change is not allowed.",
        }
    }
}

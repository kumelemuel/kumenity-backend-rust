use super::error_codes::{IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT};
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub enum AccountIdError {
    Invalid,
    WrongFormat,
}

impl fmt::Display for AccountIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdError::Invalid => write!(f, "Invalid account ID"),
            AccountIdError::WrongFormat => write!(f, "Wrong format for account ID"),
        }
    }
}

impl std::error::Error for AccountIdError {}

impl LayerError for AccountIdError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountIdError::Invalid => IAM_INVALID_ACCOUNT_ID,
            AccountIdError::WrongFormat => IAM_INVALID_ACCOUNT_ID_FORMAT,
        }
    }
}

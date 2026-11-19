use super::error_codes::{IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT};
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug)]
pub enum AccountIdError {
    Invalid,
    WrongFormat,
}

impl fmt::Display for AccountIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdError::Invalid => write!(f, "The account ID is invalid"),
            AccountIdError::WrongFormat => write!(f, "The account ID format is invalid"),
        }
    }
}

impl std::error::Error for AccountIdError {}

impl ErrorLayout for AccountIdError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountIdError::Invalid => IAM_INVALID_ACCOUNT_ID,
            AccountIdError::WrongFormat => IAM_INVALID_ACCOUNT_ID_FORMAT,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AccountIdError::Invalid => "The account ID is invalid.",
            AccountIdError::WrongFormat => "The account ID format is invalid.",
        }
    }
}

use super::error_codes::IAM_ACCOUNT_INVALID_VERIFICATION;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug)]
pub enum AccountError {
    InvalidVerification,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::InvalidVerification => write!(f, "Verification process is invalid"),
        }
    }
}

impl ErrorLayout for AccountError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountError::InvalidVerification => IAM_ACCOUNT_INVALID_VERIFICATION,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AccountError::InvalidVerification => "Verification process is invalid.",
        }
    }
}

use super::error_codes::{
    IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_INVALID_VERIFICATION, IAM_ACCOUNT_NOT_FOUND,
    IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
};
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub enum AccountError {
    AccountNotFound,
    EmailAlreadyExists,
    UsernameAlreadyExists,
    InvalidVerification,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::AccountNotFound => write!(f, "No account was found"),
            AccountError::EmailAlreadyExists => write!(f, "This email is already in use"),
            AccountError::UsernameAlreadyExists => write!(f, "This username is already in use"),
            AccountError::InvalidVerification => write!(f, "Verification process is invalid"),
        }
    }
}

impl std::error::Error for AccountError {}

impl LayerError for AccountError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            AccountError::AccountNotFound => IAM_ACCOUNT_NOT_FOUND,
            AccountError::EmailAlreadyExists => IAM_ACCOUNT_EMAIL_ALREADY_EXISTS,
            AccountError::UsernameAlreadyExists => IAM_ACCOUNT_USERNAME_ALREADY_EXISTS,
            AccountError::InvalidVerification => IAM_ACCOUNT_INVALID_VERIFICATION,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AccountError::AccountNotFound => "No account was found with the provided information.",
            AccountError::EmailAlreadyExists => "This email is already in use.",
            AccountError::UsernameAlreadyExists => "This username is already in use.",
            AccountError::InvalidVerification => "Verification process is invalid.",
        }
    }
}

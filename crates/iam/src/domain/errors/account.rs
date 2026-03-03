use super::error_codes::{
    IAM_ACCOUNT_EMAIL_ALREADY_EXISTS, IAM_ACCOUNT_NOT_FOUND, IAM_ACCOUNT_USERNAME_ALREADY_EXISTS
    ,
};
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub enum AccountError {
    AccountNotFound,
    EmailAlreadyExists,
    UsernameAlreadyExists,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::AccountNotFound => write!(f, "Invalid account ID"),
            AccountError::EmailAlreadyExists => write!(f, "Wrong format for account ID"),
            AccountError::UsernameAlreadyExists => write!(f, "Wrong format for account ID"),
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
        }
    }
}

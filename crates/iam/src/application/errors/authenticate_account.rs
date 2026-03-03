use super::error_codes::{IAM_CANNOT_AUTHENTICATE, IAM_LOGIN_FAILED};
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug)]
pub enum AuthenticateAccountError {
    LoginFailed,
    CannotAuthenticate,
}

impl fmt::Display for AuthenticateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthenticateAccountError::LoginFailed => write!(f, "Login failed"),
            AuthenticateAccountError::CannotAuthenticate => write!(f, "Cannot authenticate"),
        }
    }
}

impl std::error::Error for AuthenticateAccountError {}

impl LayerError for AuthenticateAccountError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Application
    }

    fn code(&self) -> &'static str {
        match self {
            AuthenticateAccountError::LoginFailed => IAM_LOGIN_FAILED,
            AuthenticateAccountError::CannotAuthenticate => IAM_CANNOT_AUTHENTICATE,
        }
    }
}

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
            AuthenticateAccountError::LoginFailed => {
                write!(f, "The username or password is incorrect")
            }
            AuthenticateAccountError::CannotAuthenticate => {
                write!(f, "This account is not allowed to sign in")
            }
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

    fn message(&self) -> &'static str {
        match self {
            AuthenticateAccountError::LoginFailed => "The username or password is incorrect.",
            AuthenticateAccountError::CannotAuthenticate => {
                "This account is not allowed to sign in at the moment."
            }
        }
    }
}

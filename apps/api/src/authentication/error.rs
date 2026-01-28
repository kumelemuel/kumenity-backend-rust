use crate::authentication::bearer::AuthHeaderError;

#[derive(Debug)]
pub enum TokenValidationError {
    Invalid,
    Expired,
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidToken,
}

impl From<AuthHeaderError> for AuthError {
    fn from(_: AuthHeaderError) -> Self {
        AuthError::MissingCredentials
    }
}

impl From<TokenValidationError> for AuthError {
    fn from(_: TokenValidationError) -> Self {
        AuthError::InvalidToken
    }
}

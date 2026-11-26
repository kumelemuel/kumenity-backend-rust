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

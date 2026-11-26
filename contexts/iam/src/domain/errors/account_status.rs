use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum AccountStatusError {
    InvalidTransition,
}

impl fmt::Display for AccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountStatusError::InvalidTransition => {
                write!(f, "This account status change is not allowed")
            }
        }
    }
}

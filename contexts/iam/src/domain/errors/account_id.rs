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

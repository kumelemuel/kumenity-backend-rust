use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeValidationError {
    Invalid,
}

impl fmt::Display for CodeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeValidationError::Invalid => write!(f, "The verification code is invalid"),
        }
    }
}

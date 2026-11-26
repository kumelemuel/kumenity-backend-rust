use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Invalid,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::Invalid => write!(f, "Please enter a valid email address"),
        }
    }
}

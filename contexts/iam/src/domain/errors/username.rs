use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum UsernameError {
    Invalid,
}

impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsernameError::Invalid => write!(f, "Please enter a valid username"),
        }
    }
}

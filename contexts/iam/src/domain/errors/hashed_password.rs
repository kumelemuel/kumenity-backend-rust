use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum HashedPasswordError {
    Invalid,
}

impl fmt::Display for HashedPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashedPasswordError::Invalid => write!(f, "The password format is invalid"),
        }
    }
}

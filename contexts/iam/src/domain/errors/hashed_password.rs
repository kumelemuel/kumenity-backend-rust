use super::error_codes::IAM_INVALID_HASHED_PASSWORD;
use shared::error::{ArchitectureLayer, ErrorLayout};
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

impl std::error::Error for HashedPasswordError {}

impl ErrorLayout for HashedPasswordError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            HashedPasswordError::Invalid => IAM_INVALID_HASHED_PASSWORD,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            HashedPasswordError::Invalid => "The password format is invalid.",
        }
    }
}

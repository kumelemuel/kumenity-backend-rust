use super::error_codes::{IAM_INVALID_ACCOUNT_ID, IAM_INVALID_ACCOUNT_ID_FORMAT};
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

/// Why a value is not an `OwnerId` (D2 a): not a UUID, or the nil UUID.
#[derive(Debug, PartialEq, Eq)]
pub enum InvalidOwnerId {
    Nil,
    WrongFormat,
}

impl fmt::Display for InvalidOwnerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidOwnerId::Nil => write!(f, "The account ID is invalid"),
            InvalidOwnerId::WrongFormat => write!(f, "The account ID format is invalid"),
        }
    }
}

impl ErrorLayout for InvalidOwnerId {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            InvalidOwnerId::Nil => IAM_INVALID_ACCOUNT_ID,
            InvalidOwnerId::WrongFormat => IAM_INVALID_ACCOUNT_ID_FORMAT,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            InvalidOwnerId::Nil => "The account ID is invalid.",
            InvalidOwnerId::WrongFormat => "The account ID format is invalid.",
        }
    }
}

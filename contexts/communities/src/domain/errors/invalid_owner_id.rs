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

use crate::application::errors::{
    conflict::Conflict, invalid_owner::InvalidOwner, validation::Validation,
};

/// Closed error contract (ERR-2): only the leaves this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateCommunityError {
    Validation(Validation),
    Conflict(Conflict),
    InvalidOwner(InvalidOwner),
}

impl From<Validation> for CreateCommunityError {
    fn from(leaf: Validation) -> Self {
        Self::Validation(leaf)
    }
}

impl From<Conflict> for CreateCommunityError {
    fn from(leaf: Conflict) -> Self {
        Self::Conflict(leaf)
    }
}

impl From<InvalidOwner> for CreateCommunityError {
    fn from(leaf: InvalidOwner) -> Self {
        Self::InvalidOwner(leaf)
    }
}

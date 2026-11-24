use crate::application::errors::{conflict::Conflict, validation::Validation};

/// Closed error contract (ERR-2): only the leaves this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterAccountError {
    Validation(Validation),
    Conflict(Conflict),
}

impl From<Validation> for RegisterAccountError {
    fn from(leaf: Validation) -> Self {
        Self::Validation(leaf)
    }
}

impl From<Conflict> for RegisterAccountError {
    fn from(leaf: Conflict) -> Self {
        Self::Conflict(leaf)
    }
}

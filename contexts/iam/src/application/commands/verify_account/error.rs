use crate::application::errors::{
    invalid_verification::InvalidVerification, not_found::NotFound, validation::Validation,
};

/// Closed error contract (ERR-2): only the leaves this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyAccountError {
    NotFound(NotFound),
    Validation(Validation),
    InvalidVerification(InvalidVerification),
}

impl From<NotFound> for VerifyAccountError {
    fn from(leaf: NotFound) -> Self {
        Self::NotFound(leaf)
    }
}

impl From<Validation> for VerifyAccountError {
    fn from(leaf: Validation) -> Self {
        Self::Validation(leaf)
    }
}

impl From<InvalidVerification> for VerifyAccountError {
    fn from(leaf: InvalidVerification) -> Self {
        Self::InvalidVerification(leaf)
    }
}

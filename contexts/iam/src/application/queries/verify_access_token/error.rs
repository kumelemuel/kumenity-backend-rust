use crate::application::errors::unauthenticated::Unauthenticated;

/// Closed error contract (ERR-2): the only leaf this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyAccessTokenError {
    Unauthenticated(Unauthenticated),
}

impl From<Unauthenticated> for VerifyAccessTokenError {
    fn from(leaf: Unauthenticated) -> Self {
        Self::Unauthenticated(leaf)
    }
}

use crate::application::errors::not_found::NotFound;

/// Closed error contract (ERR-2): the only leaf this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifyAccountError {
    NotFound(NotFound),
}

impl From<NotFound> for IdentifyAccountError {
    fn from(leaf: NotFound) -> Self {
        Self::NotFound(leaf)
    }
}

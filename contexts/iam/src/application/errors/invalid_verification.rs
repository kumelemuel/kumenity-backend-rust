/// Leaf (ERR-2): the account cannot be verified with this code, either
/// because it is not the one delivered or because the account is no longer
/// pending verification (D1). It does not say which.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidVerification;

/// Leaf (ERR-2): the credentials are right, but the account is not allowed
/// to sign in (it is not active).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CannotAuthenticate;

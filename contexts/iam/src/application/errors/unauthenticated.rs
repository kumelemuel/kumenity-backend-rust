/// Leaf (ERR-2): the request carries no identity that can be verified. It
/// does not say why (malformed, tampered or expired token): the reason is
/// not given to the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unauthenticated;

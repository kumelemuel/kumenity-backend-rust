use std::fmt;

/// The account exists and the password is right, but its status does not
/// allow signing in (only an active account can).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CannotAuthenticate;

impl fmt::Display for CannotAuthenticate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This account is not allowed to sign in")
    }
}

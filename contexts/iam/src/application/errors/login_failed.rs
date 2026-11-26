/// Leaf (ERR-2): the credentials do not identify an account. It does not say
/// whether the username exists or the password is wrong (D6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginFailed;

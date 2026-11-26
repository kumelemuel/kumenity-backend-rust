use crate::application::errors::{
    cannot_authenticate::CannotAuthenticate, login_failed::LoginFailed,
};

/// Closed error contract (ERR-2): only the leaves this use case produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticateAccountError {
    LoginFailed(LoginFailed),
    CannotAuthenticate(CannotAuthenticate),
}

impl From<LoginFailed> for AuthenticateAccountError {
    fn from(leaf: LoginFailed) -> Self {
        Self::LoginFailed(leaf)
    }
}

impl From<CannotAuthenticate> for AuthenticateAccountError {
    fn from(leaf: CannotAuthenticate) -> Self {
        Self::CannotAuthenticate(leaf)
    }
}

use crate::application::commands::authenticate_account::AuthenticateAccountCommand;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

/// By hand, so a future `{:?}` (a log, a `TraceLayer` with the body) never
/// prints the password (ERR-4.10).
impl fmt::Debug for SignInRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignInRequest")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

impl From<SignInRequest> for AuthenticateAccountCommand {
    fn from(req: SignInRequest) -> Self {
        AuthenticateAccountCommand {
            username: req.username,
            password: req.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_reveal_the_password() {
        let request = SignInRequest {
            username: "ana".to_string(),
            password: "s3cret-password".to_string(),
        };

        let debug = format!("{request:?}");

        assert!(!debug.contains("s3cret-password"), "{debug}");
        assert!(debug.contains("ana"), "{debug}");
    }
}

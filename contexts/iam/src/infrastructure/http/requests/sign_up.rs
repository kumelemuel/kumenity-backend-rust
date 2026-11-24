use crate::application::commands::register_account::RegisterAccountCommand;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// By hand, so a future `{:?}` (a log, a `TraceLayer` with the body) never
/// prints the password (ERR-4.10).
impl fmt::Debug for SignUpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignUpRequest")
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"<redacted>")
            .finish()
    }
}

impl From<SignUpRequest> for RegisterAccountCommand {
    fn from(req: SignUpRequest) -> Self {
        RegisterAccountCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_reveal_the_password() {
        let request = SignUpRequest {
            username: "ana".to_string(),
            email: "ana@example.com".to_string(),
            password: "s3cret-password".to_string(),
        };

        let debug = format!("{request:?}");

        assert!(!debug.contains("s3cret-password"), "{debug}");
        assert!(debug.contains("ana@example.com"), "{debug}");
    }
}

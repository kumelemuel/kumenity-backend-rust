use crate::application::commands::verify_account::VerifyAccountCommand;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub validation_code: u32,
}

/// By hand, so a future `{:?}` (a log, a `TraceLayer` with the body) never
/// prints the code that verifies the account (ERR-4.14).
impl fmt::Debug for VerifyRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifyRequest")
            .field("email", &self.email)
            .field("validation_code", &"<redacted>")
            .finish()
    }
}

impl From<VerifyRequest> for VerifyAccountCommand {
    fn from(req: VerifyRequest) -> Self {
        VerifyAccountCommand {
            email: req.email,
            code: req.validation_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_reveal_the_validation_code() {
        let request = VerifyRequest {
            email: "ana@example.com".to_string(),
            validation_code: 481_516,
        };

        let debug = format!("{request:?}");

        assert!(!debug.contains("481516"), "{debug}");
        assert!(debug.contains("ana@example.com"), "{debug}");
    }
}

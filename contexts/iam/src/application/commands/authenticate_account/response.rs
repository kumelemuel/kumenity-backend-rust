use std::fmt;

pub struct AuthenticateAccountResponse {
    pub token: String,
}

/// By hand, so a future `{:?}` never prints the access token (ERR-4.11).
impl fmt::Debug for AuthenticateAccountResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthenticateAccountResponse")
            .field("token", &"<redacted>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_reveal_the_token() {
        let response = AuthenticateAccountResponse {
            token: "header.payload.signature".to_string(),
        };

        let debug = format!("{response:?}");

        assert!(!debug.contains("header.payload.signature"), "{debug}");
    }
}

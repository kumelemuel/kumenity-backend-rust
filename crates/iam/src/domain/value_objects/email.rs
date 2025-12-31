use crate::domain::errors::invalid_email::InvalidEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidEmail> {
        let normalized = value.into().trim().to_lowercase();

        if normalized.is_empty() {
            return Err(InvalidEmail);
        }

        if !Self::is_valid_format(&normalized) {
            return Err(InvalidEmail);
        }

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid_format(value: &str) -> bool {
        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return false;
        }

        let (local, domain) = (parts[0], parts[1]);

        if local.is_empty() || domain.is_empty() {
            return false;
        }

        domain.contains('.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::InvalidEmail;

    #[test]
    fn creates_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn trims_and_lowercases_email() {
        let email = Email::new("  USER@Example.COM  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn rejects_empty_email() {
        let result = Email::new("   ");
        assert_eq!(result, Err(InvalidEmail));
    }

    #[test]
    fn rejects_missing_at_symbol() {
        let result = Email::new("invalid-email");
        assert_eq!(result, Err(InvalidEmail));
    }

    #[test]
    fn rejects_missing_domain() {
        let result = Email::new("user@");
        assert_eq!(result, Err(InvalidEmail));
    }

    #[test]
    fn rejects_missing_dot_in_domain() {
        let result = Email::new("user@example");
        assert_eq!(result, Err(InvalidEmail));
    }

    #[test]
    fn equal_emails_have_same_value() {
        let e1 = Email::new("USER@EXAMPLE.COM").unwrap();
        let e2 = Email::new("user@example.com").unwrap();

        assert_eq!(e1, e2);
    }
}

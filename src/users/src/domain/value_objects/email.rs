#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    InvalidFormat(String),
}

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into().trim().to_lowercase();

        if value.is_empty() {
            return Err(EmailError::Empty);
        }

        if !value.contains('@') || !value.contains('.') {
            return Err(EmailError::InvalidFormat(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::email::{Email,EmailError};

    #[test]
    fn should_create_valid_email() {
        let email = Email::new("user@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_str(), "user@example.com");
    }

    #[test]
    fn should_fail_with_empty_string() {
        let email = Email::new("");
        assert_eq!(email.unwrap_err(), EmailError::Empty);
    }

    #[test]
    fn should_fail_with_invalid_format() {
        let email = Email::new("invalid-email");
        assert!(matches!(email, Err(EmailError::InvalidFormat(_))));
    }

    #[test]
    fn should_normalize_to_lowercase_and_trim() {
        let email = Email::new("   USER@Example.COM   ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }
}
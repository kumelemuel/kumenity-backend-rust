use crate::domain::errors::EmailError;
use shared::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl ValueObject for Email {
    type Value = String;
    type Error = EmailError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_lowercase();

        if normalized.is_empty() {
            return Err(EmailError::Invalid);
        }

        if !Self::is_valid_format(&normalized) {
            return Err(EmailError::Invalid);
        }

        Ok(Self(normalized))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

impl Email {
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

    #[test]
    fn creates_valid_email() {
        let email = Email::new(String::from("user@example.com")).unwrap();
        assert_eq!(email.value(), "user@example.com");
    }

    #[test]
    fn trims_and_lowercases_email() {
        let email = Email::new(String::from("  USER@Example.COM  ")).unwrap();
        assert_eq!(email.value(), "user@example.com");
    }

    #[test]
    fn rejects_empty_email() {
        let result = Email::new(String::from("   "));
        assert_eq!(result, Err(EmailError::Invalid));
    }

    #[test]
    fn rejects_missing_at_symbol() {
        let result = Email::new(String::from("invalid-email"));
        assert_eq!(result, Err(EmailError::Invalid));
    }

    #[test]
    fn rejects_missing_domain() {
        let result = Email::new(String::from("user@"));
        assert_eq!(result, Err(EmailError::Invalid));
    }

    #[test]
    fn rejects_missing_dot_in_domain() {
        let result = Email::new(String::from("user@example"));
        assert_eq!(result, Err(EmailError::Invalid));
    }

    #[test]
    fn equal_emails_have_same_value() {
        let e1 = Email::new(String::from("USER@EXAMPLE.COM")).unwrap();
        let e2 = Email::new(String::from("user@example.com")).unwrap();

        assert_eq!(e1, e2);
    }
}

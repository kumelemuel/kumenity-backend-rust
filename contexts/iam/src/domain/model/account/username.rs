use crate::domain::errors::username::UsernameError;
use shared::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl ValueObject for Username {
    type Value = String;
    type Error = UsernameError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UsernameError::Invalid);
        }

        let length = trimmed.len();
        if length < 3 || length > 32 {
            return Err(UsernameError::Invalid);
        }

        if trimmed.contains(' ') {
            return Err(UsernameError::Invalid);
        }

        Ok(Self(trimmed.to_string()))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_username() {
        let username = Username::new("john_doe".to_string());

        assert!(username.is_ok());
        assert_eq!(username.unwrap().value(), "john_doe");
    }

    #[test]
    fn trims_whitespace() {
        let username = Username::new("  alice  ".to_string()).unwrap();

        assert_eq!(username.value(), "alice");
    }

    #[test]
    fn rejects_empty_username() {
        let result = Username::new("   ".to_string());

        assert_eq!(result, Err(UsernameError::Invalid));
    }

    #[test]
    fn rejects_too_short_username() {
        let result = Username::new("ab".to_string());

        assert_eq!(result, Err(UsernameError::Invalid));
    }

    #[test]
    fn rejects_too_long_username() {
        let value = "a".repeat(33);
        let result = Username::new(value);

        assert_eq!(result, Err(UsernameError::Invalid));
    }

    #[test]
    fn rejects_username_with_spaces() {
        let result = Username::new("john doe".to_string());

        assert_eq!(result, Err(UsernameError::Invalid));
    }

    #[test]
    fn usernames_with_same_value_are_equal() {
        let u1 = Username::new("bob".to_string()).unwrap();
        let u2 = Username::new("bob".to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}

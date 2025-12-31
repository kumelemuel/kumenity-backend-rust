use crate::domain::errors::InvalidUsername;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, InvalidUsername> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InvalidUsername);
        }

        let length = trimmed.len();
        if length < 3 || length > 32 {
            return Err(InvalidUsername);
        }

        if trimmed.contains(' ') {
            return Err(InvalidUsername);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::InvalidUsername;

    #[test]
    fn creates_valid_username() {
        let username = Username::new("john_doe".to_string());

        assert!(username.is_ok());
        assert_eq!(username.unwrap().as_str(), "john_doe");
    }

    #[test]
    fn trims_whitespace() {
        let username = Username::new("  alice  ".to_string()).unwrap();

        assert_eq!(username.as_str(), "alice");
    }

    #[test]
    fn rejects_empty_username() {
        let result = Username::new("   ".to_string());

        assert_eq!(result, Err(InvalidUsername));
    }

    #[test]
    fn rejects_too_short_username() {
        let result = Username::new("ab".to_string());

        assert_eq!(result, Err(InvalidUsername));
    }

    #[test]
    fn rejects_too_long_username() {
        let value = "a".repeat(33);
        let result = Username::new(value);

        assert_eq!(result, Err(InvalidUsername));
    }

    #[test]
    fn rejects_username_with_spaces() {
        let result = Username::new("john doe".to_string());

        assert_eq!(result, Err(InvalidUsername));
    }

    #[test]
    fn usernames_with_same_value_are_equal() {
        let u1 = Username::new("bob".to_string()).unwrap();
        let u2 = Username::new("bob".to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}

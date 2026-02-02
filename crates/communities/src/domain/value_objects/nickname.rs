use crate::domain::errors::invalid_nickname::InvalidNickname;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nickname(String);

impl Nickname {
    pub fn new(value: String) -> Result<Self, InvalidNickname> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InvalidNickname);
        }

        let length = trimmed.len();
        if length < 3 || length > 32 {
            return Err(InvalidNickname);
        }

        if trimmed.contains(' ') {
            return Err(InvalidNickname);
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

    #[test]
    fn creates_valid_nickname() {
        let nickname = Nickname::new("john_doe".to_string());

        assert!(nickname.is_ok());
        assert_eq!(nickname.unwrap().as_str(), "john_doe");
    }

    #[test]
    fn trims_whitespace() {
        let nickname = Nickname::new("  alice  ".to_string()).unwrap();

        assert_eq!(nickname.as_str(), "alice");
    }

    #[test]
    fn rejects_empty_nickname() {
        let result = Nickname::new("   ".to_string());

        assert_eq!(result, Err(InvalidNickname));
    }

    #[test]
    fn rejects_too_short_nickname() {
        let result = Nickname::new("ab".to_string());

        assert_eq!(result, Err(InvalidNickname));
    }

    #[test]
    fn rejects_too_long_nickname() {
        let value = "a".repeat(33);
        let result = Nickname::new(value);

        assert_eq!(result, Err(InvalidNickname));
    }

    #[test]
    fn rejects_nickname_with_spaces() {
        let result = Nickname::new("john doe".to_string());

        assert_eq!(result, Err(InvalidNickname));
    }

    #[test]
    fn nicknames_with_same_value_are_equal() {
        let u1 = Nickname::new("bob".to_string()).unwrap();
        let u2 = Nickname::new("bob".to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}

use crate::domain::errors::invalid_community_name::InvalidCommunityName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommunityName(String);

impl CommunityName {
    pub fn new(value: String) -> Result<Self, InvalidCommunityName> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InvalidCommunityName);
        }

        let length = trimmed.len();
        if length < 5 || length > 64 {
            return Err(InvalidCommunityName);
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
    fn creates_valid_community_name() {
        let community_name = CommunityName::new("Community Name".to_string());

        assert!(community_name.is_ok());
        assert_eq!(community_name.unwrap().as_str(), "Community Name");
    }

    #[test]
    fn trims_whitespace() {
        let community_name = CommunityName::new("  Community Name  ".to_string()).unwrap();

        assert_eq!(community_name.as_str(), "Community Name");
    }

    #[test]
    fn rejects_empty_community_name() {
        let result = CommunityName::new("   ".to_string());

        assert_eq!(result, Err(InvalidCommunityName));
    }

    #[test]
    fn rejects_too_short_community_name() {
        let result = CommunityName::new("abcd".to_string());

        assert_eq!(result, Err(InvalidCommunityName));
    }

    #[test]
    fn rejects_too_long_community_name() {
        let value = "a".repeat(65);
        let result = CommunityName::new(value);

        assert_eq!(result, Err(InvalidCommunityName));
    }

    #[test]
    fn community_names_with_same_value_are_equal() {
        let u1 = CommunityName::new("Community Name".to_string()).unwrap();
        let u2 = CommunityName::new("Community Name".to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}

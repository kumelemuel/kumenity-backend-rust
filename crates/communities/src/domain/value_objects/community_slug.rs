use crate::domain::errors::invalid_community_slug::InvalidCommunitySlug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommunitySlug(String);

impl CommunitySlug {
    pub fn new(value: String) -> Result<Self, InvalidCommunitySlug> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InvalidCommunitySlug);
        }

        let length = trimmed.len();
        if length < 3 || length > 32 {
            return Err(InvalidCommunitySlug);
        }

        let normalized = trimmed.to_ascii_lowercase();

        if !normalized
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c == '-')
        {
            return Err(InvalidCommunitySlug);
        }

        if normalized.starts_with('-') || normalized.ends_with('-') {
            return Err(InvalidCommunitySlug);
        }

        if normalized.contains("--") {
            return Err(InvalidCommunitySlug);
        }

        Ok(Self(normalized.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_community_slug() {
        let community_slug = CommunitySlug::new("John-Doe".to_string());

        assert!(community_slug.is_ok());
        assert_eq!(community_slug.unwrap().as_str(), "john-doe");
    }

    #[test]
    fn trims_whitespace() {
        let community_slug = CommunitySlug::new("  alice  ".to_string()).unwrap();

        assert_eq!(community_slug.as_str(), "alice");
    }

    #[test]
    fn rejects_non_alphabetic_community_slug() {
        let result = CommunitySlug::new("   ".to_string());
        let result2 = CommunitySlug::new("test 123".to_string());
        let result3 = CommunitySlug::new("test123".to_string());
        let result4 = CommunitySlug::new("test@!%".to_string());

        assert_eq!(result, Err(InvalidCommunitySlug));
        assert_eq!(result2, Err(InvalidCommunitySlug));
        assert_eq!(result3, Err(InvalidCommunitySlug));
        assert_eq!(result4, Err(InvalidCommunitySlug));
    }

    #[test]
    fn rejects_double_hyphen_community_slug() {
        let result = CommunitySlug::new("test--123".to_string());

        assert_eq!(result, Err(InvalidCommunitySlug));
    }

    #[test]
    fn rejects_hyphen_on_start_or_end_community_slug() {
        let result = CommunitySlug::new("-test123".to_string());
        let result2 = CommunitySlug::new("test123-".to_string());

        assert_eq!(result, Err(InvalidCommunitySlug));
        assert_eq!(result2, Err(InvalidCommunitySlug));
    }

    #[test]
    fn rejects_too_short_community_slug() {
        let result = CommunitySlug::new("ab".to_string());

        assert_eq!(result, Err(InvalidCommunitySlug));
    }

    #[test]
    fn rejects_too_long_community_slug() {
        let value = "a".repeat(33);
        let result = CommunitySlug::new(value);

        assert_eq!(result, Err(InvalidCommunitySlug));
    }

    #[test]
    fn rejects_community_slug_with_spaces() {
        let result = CommunitySlug::new("john doe".to_string());

        assert_eq!(result, Err(InvalidCommunitySlug));
    }

    #[test]
    fn community_slugs_with_same_value_are_equal() {
        let u1 = CommunitySlug::new("bob".to_string()).unwrap();
        let u2 = CommunitySlug::new("bob".to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}

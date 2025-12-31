use crate::domain::errors::InvalidHashedPassword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_hash(value: impl Into<String>) -> Result<Self, InvalidHashedPassword> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(InvalidHashedPassword);
        }

        if !Self::looks_like_a_hash(&value) {
            return Err(InvalidHashedPassword);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn looks_like_a_hash(value: &str) -> bool {
        value.len() >= 50
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::InvalidHashedPassword;

    impl HashedPassword {
        pub fn dummy() -> Self {
            HashedPassword::from_hash(
                "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012"
            )
                .unwrap()
        }
    }

    #[test]
    fn creates_valid_hashed_password() {
        let hash = HashedPassword::from_hash(
            "$argon2id$v=19$m=65536,t=2,p=1$hashhashhashhashhashhashhashhashhash",
        );

        assert!(hash.is_ok());
    }

    #[test]
    fn rejects_empty_hash() {
        let result = HashedPassword::from_hash("");
        assert_eq!(result, Err(InvalidHashedPassword));
    }

    #[test]
    fn rejects_whitespace_hash() {
        let result = HashedPassword::from_hash("   ");
        assert_eq!(result, Err(InvalidHashedPassword));
    }

    #[test]
    fn rejects_too_short_hash() {
        let result = HashedPassword::from_hash("short");
        assert_eq!(result, Err(InvalidHashedPassword));
    }

    #[test]
    fn equal_hashes_are_equal() {
        let h1 = HashedPassword::from_hash("x".repeat(60)).unwrap();
        let h2 = HashedPassword::from_hash("x".repeat(60)).unwrap();

        assert_eq!(h1, h2);
    }
}

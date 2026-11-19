use crate::domain::errors::hashed_password::HashedPasswordError;
use shared::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword(String);

impl ValueObject for HashedPassword {
    type Value = String;
    type Error = HashedPasswordError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(HashedPasswordError::Invalid);
        }

        if !Self::looks_like_a_hash(&value) {
            return Err(HashedPasswordError::Invalid);
        }

        Ok(Self(value))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

impl HashedPassword {
    fn looks_like_a_hash(value: &str) -> bool {
        value.len() >= 50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_hashed_password() {
        let hash = HashedPassword::new(String::from(
            "$argon2id$v=19$m=65536,t=2,p=1$hashhashhashhashhashhashhashhashhash",
        ));

        assert!(hash.is_ok());
    }

    #[test]
    fn rejects_empty_hash() {
        let result = HashedPassword::new(String::from(""));
        assert_eq!(result, Err(HashedPasswordError::Invalid));
    }

    #[test]
    fn rejects_whitespace_hash() {
        let result = HashedPassword::new(String::from("   "));
        assert_eq!(result, Err(HashedPasswordError::Invalid));
    }

    #[test]
    fn rejects_too_short_hash() {
        let result = HashedPassword::new(String::from("short"));
        assert_eq!(result, Err(HashedPasswordError::Invalid));
    }

    #[test]
    fn equal_hashes_are_equal() {
        let h1 = HashedPassword::new("x".repeat(60)).unwrap();
        let h2 = HashedPassword::new("x".repeat(60)).unwrap();

        assert_eq!(h1, h2);
    }
}

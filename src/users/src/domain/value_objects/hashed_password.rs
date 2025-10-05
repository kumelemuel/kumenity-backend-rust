#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword(String);

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordHashError {
    Empty,
    TooShort,
}

impl HashedPassword {
    pub fn new(value: impl Into<String>) -> Result<Self, PasswordHashError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PasswordHashError::Empty);
        }

        if value.len() < 60 {
            // hashes bcrypt/argon2 suelen ser largos
            return Err(PasswordHashError::TooShort);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::super::hashed_password::{HashedPassword, PasswordHashError};

    #[test]
    fn should_create_valid_password_hash() {
        let hash = HashedPassword::new("$argon2id$v=19$m=65536,t=2,p=1$hashhashhashhashhashhashhashhashhash");
        assert!(hash.is_ok());
    }

    #[test]
    fn should_fail_with_empty_string() {
        let hash = HashedPassword::new("");
        assert_eq!(hash.unwrap_err(), PasswordHashError::Empty);
    }

    #[test]
    fn should_fail_with_too_short_hash() {
        let hash = HashedPassword::new("short");
        assert_eq!(hash.unwrap_err(), PasswordHashError::TooShort);
    }
}

#[cfg(test)]
impl HashedPassword {
    pub fn dummy() -> Self {
        HashedPassword::new(
            "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string()
        ).unwrap()
    }
}
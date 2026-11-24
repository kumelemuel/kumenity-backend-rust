use shared::Fault;

/// Hashes passwords and checks a password against a stored hash.
///
/// The port declares no errors (ERR-3): a mismatch is `Ok(false)`, an answer;
/// any technical failure (the algorithm fails, the stored hash is unreadable)
/// is a `Fault`.
pub trait PasswordHashingService: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, Fault>;
    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::password_hasher::PasswordHashingService;
    use shared::Fault;

    pub struct FakePasswordHasher;

    impl PasswordHashingService for FakePasswordHasher {
        fn hash(&self, _raw: &str) -> Result<String, Fault> {
            Ok(String::from(
                "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012",
            ))
        }

        fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
            Ok(password == hashed_password)
        }
    }
}

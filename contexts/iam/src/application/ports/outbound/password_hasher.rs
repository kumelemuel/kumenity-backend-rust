pub trait PasswordHasherPort: Send + Sync {
    fn hash(&self, password: &str) -> String;
    fn verify(&self, password: &str, hashed_password: &str) -> bool;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::password_hasher::PasswordHasherPort;

    pub struct FakePasswordHasher;

    impl PasswordHasherPort for FakePasswordHasher {
        fn hash(&self, _raw: &str) -> String {
            String::from(
                "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012",
            )
        }

        fn verify(&self, password: &str, hashed_password: &str) -> bool {
            password == hashed_password
        }
    }
}

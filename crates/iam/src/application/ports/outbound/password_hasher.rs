use crate::domain::value_objects::HashedPassword;

pub trait PasswordHasherPort: Send + Sync {
    fn hash(&self, password: &str) -> HashedPassword;
    fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
    use crate::domain::value_objects::HashedPassword;

    pub struct FakePasswordHasher;

    impl PasswordHasherPort for FakePasswordHasher {
        fn hash(&self, _raw: &str) -> HashedPassword {
            HashedPassword::dummy()
        }

        fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool {
            password == hashed_password.as_str()
        }
    }
}
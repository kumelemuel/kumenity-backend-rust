use crate::domain::value_objects::HashedPassword;

pub trait PasswordHasherPort: Send + Sync {
    fn hash(&self, password: &str) -> HashedPassword;
    fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool;
}

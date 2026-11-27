use crate::domain::model::account::hashed_password::HashedPassword;
use shared::Fault;

/// Hashes passwords and checks a password against a stored hash.
///
/// The port declares no errors (ERR-3): a mismatch is `Ok(false)`, an answer;
/// any technical failure (the algorithm fails, the stored hash is unreadable)
/// is a `Fault`.
///
/// `hash` answers a `HashedPassword`: a hash that is not one cannot reach
/// the account (TDD-2.7).
pub trait PasswordHashingService: Send + Sync {
    fn hash(&self, password: &str) -> Result<HashedPassword, Fault>;
    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault>;
}

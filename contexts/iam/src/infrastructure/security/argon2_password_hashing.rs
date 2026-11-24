use crate::application::ports::outbound::password_hashing_service::PasswordHashingService;
use argon2::{
    Argon2,
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;
use shared::Fault;

/// `PasswordHashingService` with Argon2 (default parameters, random salt).
///
/// Any Argon2 error is a `Fault` (RS-2.4): hashing that fails, or a stored
/// hash it cannot read, which is corrupt persisted data. A password that
/// does not match is an answer, `Ok(false)`.
pub struct Argon2PasswordHashingAdapter {
    argon2: Argon2<'static>,
}

impl Default for Argon2PasswordHashingAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Argon2PasswordHashingAdapter {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHashingService for Argon2PasswordHashingAdapter {
    fn hash(&self, raw_password: &str) -> Result<String, Fault> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self
            .argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|e| fault("hashing failed", e))?
            .to_string())
    }

    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
        let parsed_hash = PasswordHash::new(hashed_password)
            .map_err(|e| fault("stored hash is unreadable", e))?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(other) => Err(fault("verification failed", other)),
        }
    }
}

/// `password_hash::Error` does not implement `std::error::Error` without the
/// crate's `std` feature, so `?` cannot convert it: the message goes into
/// the `Fault`, with context. It never carries the password or the hash.
fn fault(context: &str, error: password_hash::Error) -> Fault {
    Fault::new(format!("argon2: {context}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::password_hashing_service_contract! {
        new_service: Argon2PasswordHashingAdapter::new(),
    }
}

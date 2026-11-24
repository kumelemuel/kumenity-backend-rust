use crate::application::ports::outbound::password_hashing_service::PasswordHashingService;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;
use shared::Fault;

pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHashingService for Argon2PasswordHasher {
    fn hash(&self, raw_password: &str) -> Result<String, Fault> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self
            .argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .expect("argon2 hashing must not fail")
            .to_string())
    }

    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
        let parsed_hash =
            PasswordHash::new(hashed_password).expect("stored password hash must be valid");

        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

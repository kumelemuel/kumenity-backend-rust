use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;

pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHasherPort for Argon2PasswordHasher {
    fn hash(&self, raw_password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        self.argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .expect("argon2 hashing must not fail")
            .to_string()
    }

    fn verify(&self, password: &str, hashed_password: &str) -> bool {
        let parsed_hash =
            PasswordHash::new(hashed_password).expect("stored password hash must be valid");

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand::rngs::OsRng;

use crate::application::ports::outbound::password_hasher::PasswordHasherPort;
use crate::domain::value_objects::HashedPassword;

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
    fn hash(&self, raw_password: &str) -> HashedPassword {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self
            .argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .expect("argon2 hashing must not fail")
            .to_string();

        HashedPassword::from_hash(password_hash)
            .expect("argon2 hash must be a valid HashedPassword")
    }

    fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool {
        let parsed_hash = PasswordHash::new(hashed_password.as_str())
            .expect("stored password hash must be valid");

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

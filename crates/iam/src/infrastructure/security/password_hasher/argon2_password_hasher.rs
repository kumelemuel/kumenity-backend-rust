use crate::{
    application::ports::outbound::password_hasher::PasswordHasherPort,
    domain::value_objects::HashedPassword,
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
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
    #[expect(
        clippy::expect_used,
        reason = "invariantes de Argon2 con parámetros por defecto; el puerto aún no puede devolver Fault"
    )]
    fn hash(&self, raw_password: &str) -> HashedPassword {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self
            .argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .expect("invariante: Argon2 por defecto con sal nueva no falla")
            .to_string();

        HashedPassword::from_hash(password_hash)
            .expect("invariante: un hash PHC de Argon2 es un HashedPassword válido")
    }

    #[expect(
        clippy::expect_used,
        reason = "todo HashedPassword de producción sale de hash(); será Fault cuando haya persistencia real"
    )]
    fn verify(&self, password: &str, hashed_password: &HashedPassword) -> bool {
        let parsed_hash = PasswordHash::new(hashed_password.as_str())
            .expect("invariante: el hash guardado lo generó Argon2");

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

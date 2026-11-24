//! Contrato de `PasswordHashingService`: el comportamiento del que dependen
//! `RegisterAccount` y `AuthenticateAccount`. Lo pasan los dobles y todos los
//! adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): que la contraseña no coincida es una
//! respuesta (`Ok(false)`); un hash ilegible es una falla técnica (`Fault`).

use crate::{
    application::ports::outbound::password_hashing_service::PasswordHashingService,
    domain::model::account::hashed_password::HashedPassword,
};
use shared::domain::value_object::ValueObject;

const PASSWORD: &str = "correct horse battery staple";
const ANOTHER: &str = "correct horse battery stapler";

fn hash(service: &impl PasswordHashingService, password: &str) -> String {
    service
        .hash(password)
        .expect("contrato: hash no falla con el adaptador sano")
}

fn verify(service: &impl PasswordHashingService, password: &str, hashed: &str) -> bool {
    service
        .verify(password, hashed)
        .expect("contrato: verify no falla con un hash que produjo el adaptador")
}

/// `verify` acepta la contraseña con la que se generó el hash, y solo esa.
pub fn verify_accepts_the_hashed_password_and_rejects_another<S: PasswordHashingService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();
    let hashed = hash(&service, PASSWORD);

    assert!(verify(&service, PASSWORD, &hashed), "rechazó su contraseña");
    assert!(
        !verify(&service, ANOTHER, &hashed),
        "aceptó otra contraseña"
    );
}

/// El hash no es la contraseña en claro.
pub fn hash_is_not_the_password<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();

    assert_ne!(hash(&service, PASSWORD), PASSWORD);
}

/// El hash que produce el adaptador es un `HashedPassword` válido: el handler
/// se lo pasa tal cual a la factory de `Account`.
pub fn hash_is_a_valid_hashed_password<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();

    assert!(HashedPassword::new(hash(&service, PASSWORD)).is_ok());
}

/// Un hash que el adaptador no puede leer es una falla técnica (un dato
/// persistido corrupto), no una contraseña incorrecta.
pub fn corrupt_hash_is_a_fault<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();

    let result = service.verify(PASSWORD, "not-a-hash");

    assert!(result.is_err(), "{result:?}");
}

/// Un hash del propio adaptador al que le falta la última sección (un dato
/// persistido truncado) también es una falla técnica, aunque el formato se
/// pueda leer: no es una contraseña incorrecta.
pub fn truncated_hash_is_a_fault<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();
    let hashed = hash(&service, PASSWORD);
    let (truncated, _) = hashed
        .rsplit_once('$')
        .expect("contrato: el hash tiene secciones separadas por `$`");

    let result = service.verify(PASSWORD, truncated);

    assert!(result.is_err(), "{truncated}: {result:?}");
}

/// Genera un `#[test]` por caso del contrato para un adaptador.
#[macro_export]
macro_rules! password_hashing_service_contract {
    (new_service: $new_service:expr $(,)?) => {
        mod password_hashing_service_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::password_hashing_service as contract;

            #[test]
            fn verify_accepts_the_hashed_password_and_rejects_another() {
                contract::verify_accepts_the_hashed_password_and_rejects_another(|| $new_service);
            }

            #[test]
            fn hash_is_not_the_password() {
                contract::hash_is_not_the_password(|| $new_service);
            }

            #[test]
            fn hash_is_a_valid_hashed_password() {
                contract::hash_is_a_valid_hashed_password(|| $new_service);
            }

            #[test]
            fn corrupt_hash_is_a_fault() {
                contract::corrupt_hash_is_a_fault(|| $new_service);
            }

            #[test]
            fn truncated_hash_is_a_fault() {
                contract::truncated_hash_is_a_fault(|| $new_service);
            }
        }
    };
}

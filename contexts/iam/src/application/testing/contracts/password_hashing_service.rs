//! Contrato de `PasswordHashingService`: el comportamiento del que dependen
//! `RegisterAccount` y `AuthenticateAccount`. Lo pasan los dobles y todos los
//! adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): que la contraseña no coincida es una
//! respuesta (`Ok(false)`); un hash ilegible es una falla técnica (`Fault`).
//! Que el hash sea un `HashedPassword` lo asegura el tipo de `hash` (TDD-2.7).
//!
//! Forma async de la receta "Puerto todavía síncrono" (D12, R16b): los casos y
//! los helpers son `async fn` y la macro recibe `run`. Los helpers, y los casos
//! que llaman al puerto directamente, lo hacen sin `.await` hasta R23, que lo
//! pasa a `BoxFuture`.
#![expect(
    clippy::unused_async,
    reason = "puerto todavía síncrono: se quita en su rebanada de asincronía"
)]

use crate::{
    application::ports::outbound::password_hashing_service::PasswordHashingService,
    domain::model::account::hashed_password::HashedPassword,
};

const PASSWORD: &str = "correct horse battery staple";
const ANOTHER: &str = "correct horse battery stapler";

async fn hash(service: &impl PasswordHashingService, password: &str) -> HashedPassword {
    service
        .hash(password)
        .expect("contrato: hash no falla con el adaptador sano")
}

async fn verify(service: &impl PasswordHashingService, password: &str, hashed: &str) -> bool {
    service
        .verify(password, hashed)
        .expect("contrato: verify no falla con un hash que produjo el adaptador")
}

/// `verify` acepta la contraseña con la que se generó el hash, y solo esa.
pub async fn verify_accepts_the_hashed_password_and_rejects_another<S: PasswordHashingService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();
    let hashed = hash(&service, PASSWORD).await;

    assert!(
        verify(&service, PASSWORD, hashed.value()).await,
        "rechazó su contraseña"
    );
    assert!(
        !verify(&service, ANOTHER, hashed.value()).await,
        "aceptó otra contraseña"
    );
}

/// El hash no es la contraseña en claro.
pub async fn hash_is_not_the_password<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();

    assert_ne!(hash(&service, PASSWORD).await.value(), PASSWORD);
}

/// Un hash que el adaptador no puede leer es una falla técnica (un dato
/// persistido corrupto), no una contraseña incorrecta.
pub async fn corrupt_hash_is_a_fault<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();

    let result = service.verify(PASSWORD, "not-a-hash");

    assert!(result.is_err(), "{result:?}");
}

/// Un hash del propio adaptador al que le falta la última sección (un dato
/// persistido truncado) también es una falla técnica, aunque el formato se
/// pueda leer: no es una contraseña incorrecta.
pub async fn truncated_hash_is_a_fault<S: PasswordHashingService>(new_service: impl Fn() -> S) {
    let service = new_service();
    let hashed = hash(&service, PASSWORD).await;
    let (truncated, _) = hashed
        .value()
        .rsplit_once('$')
        .expect("contrato: el hash tiene secciones separadas por `$`");

    let result = service.verify(PASSWORD, truncated);

    assert!(result.is_err(), "{truncated}: {result:?}");
}

/// Genera un `#[test]` por caso del contrato para un adaptador.
///
/// `run`: ejecuta el future de cada caso (`block_on` para los dobles y Argon2,
/// que hoy bloquea; pasa a un runtime real en R23).
#[macro_export]
macro_rules! password_hashing_service_contract {
    (new_service: $new_service:expr, run: $run:expr $(,)?) => {
        mod password_hashing_service_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::password_hashing_service as contract;

            #[test]
            fn verify_accepts_the_hashed_password_and_rejects_another() {
                ($run)(
                    contract::verify_accepts_the_hashed_password_and_rejects_another(|| {
                        $new_service
                    }),
                );
            }

            #[test]
            fn hash_is_not_the_password() {
                ($run)(contract::hash_is_not_the_password(|| $new_service));
            }

            #[test]
            fn corrupt_hash_is_a_fault() {
                ($run)(contract::corrupt_hash_is_a_fault(|| $new_service));
            }

            #[test]
            fn truncated_hash_is_a_fault() {
                ($run)(contract::truncated_hash_is_a_fault(|| $new_service));
            }
        }
    };
}

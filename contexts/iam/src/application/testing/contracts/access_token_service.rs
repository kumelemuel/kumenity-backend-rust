//! Contrato de `AccessTokenService`: el comportamiento del que dependen
//! `AuthenticateAccount` (emite) y el middleware de autenticación (verifica).
//! Lo pasan el doble y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): un token mal formado, alterado, de
//! otro emisor o vencido es una respuesta, `Ok(Invalid)`, nunca un `Fault`.
//!
//! El resultado depende del token, no de un escenario de la fábrica. Además de
//! la fábrica, la batería recibe:
//! - `expired_token`: emite, con la misma clave del servicio, un token ya
//!   vencido (el puerto no puede emitirlo: `issue` siempre da uno vigente);
//! - `another_issuer`: un servicio sano con otra clave.
//!
//! Forma async de la receta "Puerto todavía síncrono" (D12, R16b): los casos y
//! los helpers son `async fn` y la macro recibe `run`. Los helpers, y los casos
//! que llaman al puerto directamente, lo hacen sin `.await` hasta R23, que lo
//! pasa a `BoxFuture`.
#![expect(
    clippy::unused_async,
    reason = "puerto todavía síncrono: se quita en su rebanada de asincronía"
)]

use crate::application::ports::outbound::{
    access_token_service::AccessTokenService, dtos::access_token_payloads::AccessTokenVerdict,
};

const ALICE: &str = "6f1c1b2e-3d4a-4b5c-8d9e-0f1a2b3c4d5e";
const BOB: &str = "a7b8c9d0-e1f2-4a3b-9c4d-5e6f7a8b9c0d";

async fn issue(service: &impl AccessTokenService, account_id: &str) -> String {
    service
        .issue(account_id)
        .expect("contrato: issue no falla con el servicio sano")
}

fn valid(account_id: &str) -> AccessTokenVerdict {
    AccessTokenVerdict::Valid {
        account_id: account_id.to_string(),
    }
}

/// Un token emitido por el mismo servicio es válido y trae su cuenta.
pub async fn issued_token_is_valid_for_its_account<S: AccessTokenService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();
    let token = issue(&service, ALICE).await;

    let result = service.verify(&token);

    assert_eq!(result.ok(), Some(valid(ALICE)));
}

/// Cada token trae la cuenta para la que se emitió, no la de otro.
pub async fn each_token_carries_its_own_account<S: AccessTokenService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();
    let alice = issue(&service, ALICE).await;
    let bob = issue(&service, BOB).await;

    assert_eq!(service.verify(&alice).ok(), Some(valid(ALICE)));
    assert_eq!(service.verify(&bob).ok(), Some(valid(BOB)));
}

/// Un token mal formado es un veredicto, no un `Fault` (ERR-3.1).
pub async fn malformed_token_is_an_invalid_verdict<S: AccessTokenService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();

    let result = service.verify("not-a-token");

    assert!(
        matches!(result, Ok(AccessTokenVerdict::Invalid)),
        "mal formado es Ok(Invalid): {result:?}"
    );
}

/// Un token emitido y luego alterado ya no es válido.
pub async fn tampered_token_is_an_invalid_verdict<S: AccessTokenService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();
    let mut token = issue(&service, ALICE).await;
    token.push_str("tampered");

    let result = service.verify(&token);

    assert!(
        matches!(result, Ok(AccessTokenVerdict::Invalid)),
        "alterado es Ok(Invalid): {result:?}"
    );
}

/// Un token que emitió otro servicio (otra clave) no es válido aquí, aunque
/// este también haya emitido tokens: no lo confunde con uno propio.
pub async fn token_from_another_issuer_is_an_invalid_verdict<S: AccessTokenService>(
    new_service: impl Fn() -> S,
    another_issuer: impl Fn() -> S,
) {
    let service = new_service();
    issue(&service, BOB).await;
    let token = issue(&another_issuer(), ALICE).await;

    let result = service.verify(&token);

    assert!(
        matches!(result, Ok(AccessTokenVerdict::Invalid)),
        "de otro emisor es Ok(Invalid): {result:?}"
    );
}

/// Un token vencido es un veredicto, no un `Fault` (ERR-3.1).
pub async fn expired_token_is_an_invalid_verdict<S: AccessTokenService>(
    new_service: impl Fn() -> S,
    expired_token: impl Fn(&S, &str) -> String,
) {
    let service = new_service();
    let token = expired_token(&service, ALICE);

    let result = service.verify(&token);

    assert!(
        matches!(result, Ok(AccessTokenVerdict::Invalid)),
        "vencido es Ok(Invalid): {result:?}"
    );
}

/// Genera un `#[test]` por caso del contrato para un servicio.
///
/// - `new_service`: construye el servicio. Se evalúa en un módulo hijo del que
///   invoca la macro (`use super::*`).
/// - `expired_token`: `impl Fn(&S, &str) -> String`, emite un token vencido
///   para la cuenta con la misma clave del servicio.
/// - `another_issuer`: construye un servicio sano con otra clave.
/// - `run`: ejecuta el future de cada caso (`block_on` para el doble y JWT,
///   que hoy resuelve sin E/S).
#[macro_export]
macro_rules! access_token_service_contract {
    (
        new_service: $new_service:expr,
        expired_token: $expired_token:expr,
        another_issuer: $another_issuer:expr,
        run: $run:expr $(,)?
    ) => {
        mod access_token_service_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::access_token_service as contract;

            #[test]
            fn issued_token_is_valid_for_its_account() {
                ($run)(contract::issued_token_is_valid_for_its_account(|| {
                    $new_service
                }));
            }

            #[test]
            fn each_token_carries_its_own_account() {
                ($run)(contract::each_token_carries_its_own_account(|| {
                    $new_service
                }));
            }

            #[test]
            fn malformed_token_is_an_invalid_verdict() {
                ($run)(contract::malformed_token_is_an_invalid_verdict(|| {
                    $new_service
                }));
            }

            #[test]
            fn tampered_token_is_an_invalid_verdict() {
                ($run)(contract::tampered_token_is_an_invalid_verdict(|| {
                    $new_service
                }));
            }

            #[test]
            fn token_from_another_issuer_is_an_invalid_verdict() {
                ($run)(contract::token_from_another_issuer_is_an_invalid_verdict(
                    || $new_service,
                    || $another_issuer,
                ));
            }

            #[test]
            fn expired_token_is_an_invalid_verdict() {
                ($run)(contract::expired_token_is_an_invalid_verdict(
                    || $new_service,
                    $expired_token,
                ));
            }
        }
    };
}

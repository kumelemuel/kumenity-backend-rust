//! Contrato de `VerificationCodeService`: el comportamiento del que depende
//! `RegisterAccount`. Lo pasan el doble y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): cualquier falla es `Fault`, así que no
//! hay casos por variante. El rango del código lo asegura `CodeValidation`,
//! así que la batería no lo afirma (como `HashedPassword` en la de
//! `PasswordHashingService`, TDD-2.7).
//!
//! Nace con la forma async de la receta "Puerto todavía síncrono" (§5.1 de
//! `docs/harness-adoption.md`): los casos son `async fn` y la macro recibe
//! `run`. Llaman al puerto sin `.await` hasta R23, que lo pasa a `BoxFuture`.
#![expect(
    clippy::unused_async,
    reason = "puerto todavía síncrono: se quita en su rebanada de asincronía"
)]

use crate::application::ports::outbound::verification_code_service::VerificationCodeService;

/// Un adaptador sano responde un código. Que tenga seis dígitos lo asegura
/// el tipo: `generate` responde un `CodeValidation`, que no admite otro rango.
pub async fn healthy_adapter_answers_a_code<S: VerificationCodeService>(
    new_service: impl Fn() -> S,
) {
    let service = new_service();

    let result = service.generate();

    assert!(result.is_ok(), "{result:?}");
}

/// Genera un `#[test]` por caso del contrato para un adaptador.
///
/// `run`: ejecuta el future de cada caso (`block_on` mientras el puerto sea
/// síncrono).
#[macro_export]
macro_rules! verification_code_service_contract {
    (new_service: $new_service:expr, run: $run:expr $(,)?) => {
        mod verification_code_service_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::verification_code_service as contract;

            #[test]
            fn healthy_adapter_answers_a_code() {
                ($run)(contract::healthy_adapter_answers_a_code(|| $new_service));
            }
        }
    };
}

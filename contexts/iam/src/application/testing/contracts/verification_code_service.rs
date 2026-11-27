//! Contrato de `VerificationCodeService`: el comportamiento del que depende
//! `RegisterAccount`. Lo pasan el doble y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): cualquier falla es `Fault`, así que no
//! hay casos por variante.
//!
//! Nace con la forma async de la receta "Puerto todavía síncrono" (§5.1 de
//! `docs/harness-adoption.md`): los casos son `async fn` y la macro recibe
//! `run`. Llaman al puerto sin `.await` hasta R23, que lo pasa a `BoxFuture`.
#![expect(
    clippy::unused_async,
    reason = "puerto todavía síncrono: se quita en su rebanada de asincronía"
)]

use crate::application::ports::outbound::verification_code_service::VerificationCodeService;
use shared::domain::value_object::ValueObject;

/// El código que genera tiene seis dígitos: es el que la cuenta espera en
/// `confirm_registration`.
pub async fn generates_six_digit_codes<S: VerificationCodeService>(new_service: impl Fn() -> S) {
    let service = new_service();

    let code = service
        .generate()
        .expect("contrato: generate no falla con el adaptador sano");

    assert!(
        (100_000..=999_999).contains(code.value()),
        "{}",
        code.value()
    );
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
            fn generates_six_digit_codes() {
                ($run)(contract::generates_six_digit_codes(|| $new_service));
            }
        }
    };
}

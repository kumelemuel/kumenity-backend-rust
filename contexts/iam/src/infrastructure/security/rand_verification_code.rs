use crate::{
    application::ports::outbound::verification_code_service::VerificationCodeService,
    domain::model::account::code_validation::CodeValidation,
};
use rand::Rng;
use shared::Fault;

/// `VerificationCodeService` with `rand`: a six-digit code drawn uniformly
/// from the thread's generator, seeded by the operating system.
pub struct RandVerificationCodeAdapter;

impl VerificationCodeService for RandVerificationCodeAdapter {
    fn generate(&self) -> Result<CodeValidation, Fault> {
        let code = rand::thread_rng().gen_range(100_000..=999_999);
        // The range is the one `CodeValidation` accepts.
        Ok(CodeValidation::new(code)
            .expect("invariante: un código de 100000 a 999999 es un CodeValidation"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::testing::block_on;

    crate::verification_code_service_contract! {
        new_service: RandVerificationCodeAdapter,
        run: block_on,
    }

    /// Drawn at random, not fixed: among 20 codes there are at least two
    /// different ones (all equal by chance: about 1 in 10^113).
    #[test]
    fn draws_different_codes() {
        let codes: std::collections::HashSet<CodeValidation> = (0..20)
            .map(|_| {
                RandVerificationCodeAdapter
                    .generate()
                    .expect("el adaptador sano no falla")
            })
            .collect();

        assert!(codes.len() > 1, "{codes:?}");
    }
}

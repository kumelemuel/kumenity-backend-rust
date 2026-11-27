use crate::domain::model::account::code_validation::CodeValidation;
use shared::Fault;

/// Generates the code that confirms a registration. The domain does not
/// draw it (HEX-2.3): `RegisterAccountHandler` asks this port and hands the
/// code to the account.
///
/// The port declares no errors (ERR-3): any technical failure (the source
/// of randomness fails) is a `Fault`. It answers a `CodeValidation`, so a
/// code out of range cannot reach the account.
pub trait VerificationCodeService: Send + Sync {
    fn generate(&self) -> Result<CodeValidation, Fault>;
}

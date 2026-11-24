//! Payloads de `AccessTokenService`.

/// Respuesta del verificador. `Invalid` es éxito del puerto (ERR-3): qué hacer
/// con él lo decide el caso de uso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessTokenVerdict {
    /// Firma válida y vigente; `account_id` es la cuenta a la que se emitió.
    Valid { account_id: String },
    /// Mal formado, con otra firma o vencido. El motivo no viaja: nadie
    /// ramifica por él.
    Invalid,
}

use crate::application::ports::outbound::dtos::access_token_payloads::AccessTokenVerdict;
use shared::Fault;

/// Issues access tokens for an account and verifies them (HEX-5). `Service`
/// is the last-resort role: signing and checking a token is a local
/// technical utility, neither persistence nor a third party.
///
/// The port declares no errors (ERR-3): a malformed, tampered or expired
/// token is an answer, `AccessTokenVerdict::Invalid`. Only a technical
/// failure (a misconfigured key, a clock that cannot compute the expiry) is
/// a `Fault`.
pub trait AccessTokenService: Send + Sync {
    fn issue(&self, account_id: &str) -> Result<String, Fault>;
    fn verify(&self, token: &str) -> Result<AccessTokenVerdict, Fault>;
}

use crate::application::ports::outbound::{
    access_token_service::AccessTokenService, dtos::access_token_payloads::AccessTokenVerdict,
};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};
use shared::Fault;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// `AccessTokenService` with JWTs signed with HS256: `sub` is the account id
/// and `exp` the expiry, both required.
///
/// `Fault` is only for technical failures: an expiry that overflows the
/// clock in `issue`, a key or validation that cannot check a token in
/// `verify`.
pub struct JwtAccessTokenAdapter {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    ttl_seconds: u64,
}

impl JwtAccessTokenAdapter {
    pub fn new(secret: String, ttl_seconds: u64) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["exp", "sub"]);
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation,
            ttl_seconds,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

impl AccessTokenService for JwtAccessTokenAdapter {
    fn issue(&self, account_id: &str) -> Result<String, Fault> {
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(self.ttl_seconds))
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .ok_or_else(|| Fault::new("jwt: unable to calculate expiration time"))?;

        let claims = Claims {
            sub: account_id.to_string(),
            exp,
        };

        Ok(encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &self.encoding_key,
        )?)
    }

    /// A token `decode` rejects is `Invalid` (ERR-3.1): malformed, with
    /// another signature or algorithm, expired, not yet valid or without its
    /// claims. The reason is not told apart, because nobody branches on it.
    ///
    /// Any other error is about the key or the validation, not the token: a
    /// `Fault`, so a misconfiguration is a 500 and not a 401 for everyone.
    fn verify(&self, token: &str) -> Result<AccessTokenVerdict, Fault> {
        match decode::<Claims>(token, &self.decoding_key, &self.validation) {
            Ok(data) => Ok(AccessTokenVerdict::Valid {
                account_id: data.claims.sub,
            }),
            Err(error) if is_about_the_token(error.kind()) => Ok(AccessTokenVerdict::Invalid),
            Err(error) => Err(error.into()),
        }
    }
}

fn is_about_the_token(kind: &ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::ExpiredSignature
            | ErrorKind::ImmatureSignature
            | ErrorKind::MissingRequiredClaim(_)
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::Base64(_)
            | ErrorKind::Json(_)
            | ErrorKind::Utf8(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::get_current_timestamp;

    const SECRET: &str = "test-secret";

    /// A token for `account_id` signed with `SECRET` that expired beyond the
    /// 60 s leeway of the validation.
    fn expired_token(_service: &JwtAccessTokenAdapter, account_id: &str) -> String {
        encode(
            &Header::new(Algorithm::HS256),
            &Claims {
                sub: account_id.to_string(),
                exp: get_current_timestamp() - 120,
            },
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .expect("invariante: HS256 siempre firma")
    }

    #[test]
    fn issue_is_a_fault_when_the_expiry_overflows_the_clock() {
        let service = JwtAccessTokenAdapter::new(SECRET.into(), u64::MAX);

        let result = service.issue("alice");

        assert!(result.is_err(), "{result:?}");
    }

    #[test]
    fn a_misconfigured_validation_is_a_fault_not_an_invalid_verdict() {
        let mut service = JwtAccessTokenAdapter::new(SECRET.into(), 3600);
        let token = service.issue("alice").expect("issue");
        service.validation.algorithms.clear();

        let result = service.verify(&token);

        assert!(result.is_err(), "{result:?}");
    }

    crate::access_token_service_contract! {
        new_service: JwtAccessTokenAdapter::new(SECRET.into(), 3600),
        expired_token: expired_token,
        another_issuer: JwtAccessTokenAdapter::new("another-secret".into(), 3600),
    }
}

use crate::{
    application::ports::outbound::{
        dtos::access_token_payloads::AccessTokenVerdict, token_generator::AccessTokenService,
    },
    infrastructure::services::jwt_service::claims::Claims,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use shared::Fault;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct JwtService {
    secret: String,
    ttl_seconds: u64,
}

impl JwtService {
    pub fn new(secret: String, ttl_seconds: u64) -> Self {
        Self {
            secret,
            ttl_seconds,
        }
    }
}

impl AccessTokenService for JwtService {
    fn issue(&self, account_id: &str) -> Result<String, Fault> {
        let expiration = SystemTime::now()
            .checked_add(std::time::Duration::from_secs(self.ttl_seconds))
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as usize)
            .ok_or_else(|| Fault::new("jwt: unable to calculate expiration time"))?;

        let claims = Claims {
            sub: account_id.to_string(),
            exp: expiration,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?)
    }

    // Mechanical translation of the old `TokenVerifier`: every token it
    // rejected was an `Err`, and still is. The battery of the port turns the
    // rejected tokens into `Invalid` verdicts (ERR-3.1).
    fn verify(&self, token: &str) -> Result<AccessTokenVerdict, Fault> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;

        Ok(AccessTokenVerdict::Valid {
            account_id: data.claims.sub,
        })
    }
}

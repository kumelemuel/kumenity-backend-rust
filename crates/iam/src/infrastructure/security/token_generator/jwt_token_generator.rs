use crate::{
    application::{
        errors::token_generator::TokenGeneratorError,
        ports::outbound::token_generator::TokenGeneratorPort,
    },
    infrastructure::security::token_generator::{claims::Claims, error::JwtError},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct JwtTokenGenerator {
    secret: String,
    ttl_seconds: u64,
}

impl JwtTokenGenerator {
    pub fn new(secret: String, ttl_seconds: u64) -> Self {
        Self {
            secret,
            ttl_seconds,
        }
    }
}

impl JwtTokenGenerator {
    pub fn decode(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|err| {
            use jsonwebtoken::errors::ErrorKind;

            match err.kind() {
                ErrorKind::ExpiredSignature => JwtError::Expired,
                _ => JwtError::InvalidToken,
            }
        })?;

        Ok(data.claims)
    }
}

impl TokenGeneratorPort for JwtTokenGenerator {
    fn generate(&self, user_id: &str) -> Result<String, TokenGeneratorError> {
        let expiration = SystemTime::now()
            .checked_add(std::time::Duration::from_secs(self.ttl_seconds))
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as usize)
            .ok_or("Unable to calculate expiration time")
            .map_err(|e| TokenGeneratorError(e.to_string()))?;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| TokenGeneratorError(e.to_string()))
    }
}

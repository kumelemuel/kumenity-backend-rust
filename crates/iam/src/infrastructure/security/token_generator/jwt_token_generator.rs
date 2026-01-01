use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::ports::outbound::token_generator_port::{
    TokenGeneratorPort,
};
use crate::infrastructure::security::token_generator::claims::Claims;

pub struct JwtTokenGenerator {
    secret: String,
    ttl_seconds: u64,
}

impl JwtTokenGenerator {
    pub fn new(secret: String, ttl_seconds: u64) -> Self {
        Self { secret, ttl_seconds }
    }
}

impl TokenGeneratorPort for JwtTokenGenerator {
    fn generate(&self, user_id: &str) -> Result<String, String> {
        let expiration = SystemTime::now()
            .checked_add(std::time::Duration::from_secs(self.ttl_seconds))
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as usize)
            .ok_or("Unable to calculate expiration time")?;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
            .map_err(|e| e.to_string())
    }
}

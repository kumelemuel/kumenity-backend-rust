use crate::config::error::ConfigError;

pub struct JwtConfig {
    pub secret: String,
    pub expiration_time: u64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        let expiration_time: u64 = std::env::var("JWT_EXPIRATION_TIME")
            .map_err(|_| ConfigError::Missing("JWT_EXPIRATION_TIME"))?.parse().unwrap();

        Ok(Self { secret, expiration_time })
    }
}

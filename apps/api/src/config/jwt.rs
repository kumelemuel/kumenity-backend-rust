use crate::config::error::ConfigError;

pub struct JwtConfig {
    pub secret: String,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::Missing("JWT_SECRET"))?;

        Ok(Self { secret })
    }
}

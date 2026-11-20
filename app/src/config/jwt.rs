use crate::config::error::ConfigError;
use std::fmt;

/// `Debug` está escrito a mano para no mostrar `secret` (ERR-4).
pub struct JwtConfig {
    pub secret: String,
    pub expiration_time: u64,
}

impl fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JwtConfig")
            .field("secret", &"<redacted>")
            .field("expiration_time", &self.expiration_time)
            .finish()
    }
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        let expiration_time: u64 = std::env::var("JWT_EXPIRATION_TIME")
            .map_err(|_| ConfigError::Missing("JWT_EXPIRATION_TIME"))?
            .parse()
            .unwrap();

        Ok(Self {
            secret,
            expiration_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::JwtConfig;
    use crate::config::app::AppConfig;

    const SECRET: &str = "s3cr3t-that-must-never-be-logged";

    fn jwt() -> JwtConfig {
        JwtConfig {
            secret: SECRET.to_string(),
            expiration_time: 3600,
        }
    }

    #[test]
    fn debug_does_not_reveal_the_secret() {
        let shown = format!("{:?}", jwt());

        assert!(!shown.contains(SECRET), "{shown}");
        assert!(shown.contains("3600"), "{shown}");
    }

    #[test]
    fn app_config_debug_does_not_reveal_the_secret() {
        let shown = format!("{:?}", AppConfig { jwt: jwt() });

        assert!(!shown.contains(SECRET), "{shown}");
    }
}

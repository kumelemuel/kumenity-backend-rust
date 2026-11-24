use crate::config::error::ConfigError;
use std::fmt;

/// Minimum length of `JWT_SECRET`, as `.env.example` asks.
const MIN_SECRET_BYTES: usize = 32;

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
        Self::from_vars(|name| std::env::var(name).ok())
    }

    /// Lee la configuración con `var`, que devuelve el valor de una variable.
    fn from_vars(var: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let secret = var("JWT_SECRET").ok_or(ConfigError::Missing("JWT_SECRET"))?;
        // An empty or short HS256 key lets anyone forge tokens (ERR-4).
        if secret.len() < MIN_SECRET_BYTES {
            return Err(ConfigError::Invalid("JWT_SECRET"));
        }
        let expiration_time: u64 = var("JWT_EXPIRATION_TIME")
            .ok_or(ConfigError::Missing("JWT_EXPIRATION_TIME"))?
            .parse()
            .map_err(|_| ConfigError::Invalid("JWT_EXPIRATION_TIME"))?;

        Ok(Self {
            secret,
            expiration_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::JwtConfig;
    use crate::config::{app::AppConfig, error::ConfigError};

    const SECRET: &str = "s3cr3t-that-must-never-be-logged";

    fn jwt() -> JwtConfig {
        JwtConfig {
            secret: SECRET.to_string(),
            expiration_time: 3600,
        }
    }

    fn vars(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let pairs: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.clone())
        }
    }

    #[test]
    fn reads_secret_and_expiration() {
        let config = JwtConfig::from_vars(vars(&[
            ("JWT_SECRET", SECRET),
            ("JWT_EXPIRATION_TIME", "60"),
        ]))
        .expect("valid configuration");

        assert_eq!(config.secret, SECRET);
        assert_eq!(config.expiration_time, 60);
    }

    #[test]
    fn fails_when_the_secret_is_missing() {
        let result = JwtConfig::from_vars(vars(&[("JWT_EXPIRATION_TIME", "60")]));

        assert!(matches!(result, Err(ConfigError::Missing("JWT_SECRET"))));
    }

    #[test]
    fn fails_when_the_secret_is_empty() {
        let result =
            JwtConfig::from_vars(vars(&[("JWT_SECRET", ""), ("JWT_EXPIRATION_TIME", "60")]));

        assert!(matches!(result, Err(ConfigError::Invalid("JWT_SECRET"))));
    }

    #[test]
    fn fails_when_the_secret_is_shorter_than_32_bytes() {
        let result = JwtConfig::from_vars(vars(&[
            ("JWT_SECRET", &"x".repeat(31)),
            ("JWT_EXPIRATION_TIME", "60"),
        ]));

        assert!(matches!(result, Err(ConfigError::Invalid("JWT_SECRET"))));
    }

    #[test]
    fn accepts_a_secret_of_exactly_32_bytes() {
        let secret = "x".repeat(32);
        let config = JwtConfig::from_vars(vars(&[
            ("JWT_SECRET", &secret),
            ("JWT_EXPIRATION_TIME", "60"),
        ]))
        .expect("valid configuration");

        assert_eq!(config.secret, secret);
    }

    #[test]
    fn fails_when_the_expiration_is_not_a_number() {
        let result = JwtConfig::from_vars(vars(&[
            ("JWT_SECRET", SECRET),
            ("JWT_EXPIRATION_TIME", "one hour"),
        ]));

        assert!(matches!(
            result,
            Err(ConfigError::Invalid("JWT_EXPIRATION_TIME"))
        ));
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

use crate::config::error::ConfigError;

pub struct JwtConfig {
    pub secret: String,
    pub expiration_time: u64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let secret = lookup("JWT_SECRET").ok_or(ConfigError::Missing("JWT_SECRET"))?;
        let expiration_time: u64 = lookup("JWT_EXPIRATION_TIME")
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
    use super::*;

    fn lookup(vars: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |key| {
            vars.iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn reads_secret_and_expiration_time() {
        let config = JwtConfig::from_lookup(lookup(&[
            ("JWT_SECRET", "s3cr3t"),
            ("JWT_EXPIRATION_TIME", "3600"),
        ]))
        .unwrap();

        assert_eq!(config.secret, "s3cr3t");
        assert_eq!(config.expiration_time, 3600);
    }

    #[test]
    fn fails_when_secret_is_missing() {
        let result = JwtConfig::from_lookup(lookup(&[("JWT_EXPIRATION_TIME", "3600")]));

        assert!(matches!(result, Err(ConfigError::Missing("JWT_SECRET"))));
    }

    #[test]
    fn fails_when_expiration_time_is_missing() {
        let result = JwtConfig::from_lookup(lookup(&[("JWT_SECRET", "s3cr3t")]));

        assert!(matches!(
            result,
            Err(ConfigError::Missing("JWT_EXPIRATION_TIME"))
        ));
    }

    #[test]
    fn fails_when_expiration_time_is_not_a_number() {
        let result = JwtConfig::from_lookup(lookup(&[
            ("JWT_SECRET", "s3cr3t"),
            ("JWT_EXPIRATION_TIME", "one hour"),
        ]));

        assert!(matches!(
            result,
            Err(ConfigError::Invalid("JWT_EXPIRATION_TIME"))
        ));
    }
}

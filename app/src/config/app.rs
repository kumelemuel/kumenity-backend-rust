use crate::config::{error::ConfigError, jwt::JwtConfig};

#[derive(Debug)]
pub struct AppConfig {
    pub(crate) jwt: JwtConfig,
}

impl AppConfig {
    pub fn load() -> Result<AppConfig, ConfigError> {
        dotenvy::dotenv().ok();
        let jwt = JwtConfig::from_env().unwrap_or_else(|e| {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        });
        Ok(Self { jwt })
    }
}

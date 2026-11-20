use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConfigError {
    Missing(&'static str),
    Invalid(&'static str),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
            ConfigError::Invalid(var) => {
                write!(f, "Invalid value for environment variable: {}", var)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

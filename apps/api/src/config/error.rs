use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConfigError {
    Missing(&'static str),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
        }
    }
}

impl std::error::Error for ConfigError {}
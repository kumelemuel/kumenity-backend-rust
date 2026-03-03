use std::fmt::Debug;

#[derive(Debug)]
pub enum ErrorCategory {
    Domain,
    Application,
    Infrastructure,
}
pub trait LayerError: std::error::Error + Send + Sync {
    fn category(&self) -> ErrorCategory;
    fn code(&self) -> &'static str;
}

#[derive(Debug)]
pub enum SystemError {
    Domain(Box<dyn LayerError>),
    Application(Box<dyn LayerError>),
    Infrastructure(Box<dyn LayerError>),
}

impl SystemError {
    pub fn code(&self) -> &'static str {
        match self {
            SystemError::Domain(err) => err.code(),
            SystemError::Application(err) => err.code(),
            SystemError::Infrastructure(err) => err.code(),
        }
    }
}

impl<T> From<T> for SystemError
where
    T: LayerError + 'static,
{
    fn from(error: T) -> Self {
        match error.category() {
            ErrorCategory::Domain => SystemError::Domain(Box::new(error)),
            ErrorCategory::Infrastructure => SystemError::Infrastructure(Box::new(error)),
            ErrorCategory::Application => SystemError::Application(Box::new(error)),
        }
    }
}

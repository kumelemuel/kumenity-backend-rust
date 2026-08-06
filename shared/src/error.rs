use std::{error::Error, fmt::Debug};

#[derive(Debug)]
pub enum ArchitectureLayer {
    Domain,
    Application,
    Infrastructure,
}
pub trait ErrorLayout: Error + Send + Sync {
    fn architecture_layer(&self) -> ArchitectureLayer;
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

#[derive(Debug)]
pub enum SystemError {
    Domain(Box<dyn ErrorLayout>),
    Application(Box<dyn ErrorLayout>),
    Infrastructure(Box<dyn ErrorLayout>),
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
    T: ErrorLayout + 'static,
{
    fn from(error: T) -> Self {
        match error.architecture_layer() {
            ArchitectureLayer::Domain => SystemError::Domain(Box::new(error)),
            ArchitectureLayer::Infrastructure => SystemError::Infrastructure(Box::new(error)),
            ArchitectureLayer::Application => SystemError::Application(Box::new(error)),
        }
    }
}

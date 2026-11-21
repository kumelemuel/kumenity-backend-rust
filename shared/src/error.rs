use crate::Fault;
use std::fmt::Debug;

#[derive(Debug)]
pub enum ArchitectureLayer {
    Domain,
    Application,
    Infrastructure,
}
pub trait ErrorLayout: Debug + Send + Sync {
    fn architecture_layer(&self) -> ArchitectureLayer;
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

#[derive(Debug)]
pub enum SystemError {
    Domain(Box<dyn ErrorLayout>),
    Application(Box<dyn ErrorLayout>),
    Infrastructure(Box<dyn ErrorLayout>),
    /// Puente temporal hacia el árbol B (ERR-1): un `Fault` viaja por aquí hasta
    /// que cada caso de uso tenga su `Failure<{Verb}{Noun}Error>`. Se borra en R19.
    Unexpected(Fault),
}

impl SystemError {
    /// Código de D7 para `Unexpected`: el borde lo usa en el cuerpo del 500.
    pub const UNEXPECTED_CODE: &'static str = "INTERNAL_ERROR";

    pub fn code(&self) -> &'static str {
        match self {
            SystemError::Domain(err) => err.code(),
            SystemError::Application(err) => err.code(),
            SystemError::Infrastructure(err) => err.code(),
            SystemError::Unexpected(_) => Self::UNEXPECTED_CODE,
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

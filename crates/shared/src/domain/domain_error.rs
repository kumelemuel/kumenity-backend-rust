use std::fmt::{Debug, Display};

pub trait DomainError: Debug + Display + Send + Sync + 'static {
    fn code(&self) -> &'static str;
}

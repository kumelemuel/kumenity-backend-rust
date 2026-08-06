use std::fmt::{Debug, Display};

pub trait InfrastructureError: Debug + Display + Send + Sync + 'static {
    fn code(&self) -> &'static str;
}

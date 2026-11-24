pub mod domain;
pub mod error;
pub mod infrastructure;

mod failure;
mod fault;
mod id;

pub use failure::Failure;
pub use fault::{Fault, FaultReporter};
pub use id::IdGenerator;

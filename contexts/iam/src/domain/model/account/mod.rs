// RS-1.1: se aplana al separar el dominio en su propio crate (docs/harness-adoption.md)
#[allow(clippy::module_inception)]
pub mod account;
pub mod account_factory;
pub mod account_id;
pub mod account_status;
pub mod code_validation;
pub mod email;
pub mod events;
pub mod hashed_password;
pub mod username;

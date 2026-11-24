//! Baterías de contrato de los puertos salientes (TDD-2).
//!
//! Una por puerto. Cada caso es una función que recibe la fábrica del
//! adaptador, y la macro de cada batería genera un `#[test]` por caso. Los
//! puertos todavía son síncronos: cuando pasen a `BoxFuture` (R23), cada caso
//! pasa a `async fn` y la macro recibe el `run` del caller, como en el template.

pub mod account_repository;

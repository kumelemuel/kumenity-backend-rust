//! Stable client codes (ERR-4.4): transport vocabulary, so they live in the
//! HTTP adapter and not in another context's domain (HEX-1.4). Byte for byte
//! the ones the characterization tests fix (`app/tests/communities_api.rs`).
//!
//! D2 a): an invalid owner keeps the `IAM_` codes. The rest of the
//! communities codes come here with the exhaustive `fold` of
//! `CreateCommunity` (R17).

pub const IAM_INVALID_ACCOUNT_ID: &str = "IAM_INVALID_ACCOUNT_ID";
pub const IAM_INVALID_ACCOUNT_ID_FORMAT: &str = "IAM_INVALID_ACCOUNT_ID_FORMAT";

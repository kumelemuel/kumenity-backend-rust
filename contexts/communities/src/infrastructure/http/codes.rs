//! Stable client codes (ERR-4.4): transport vocabulary, so they live in the
//! HTTP adapter and not in another context's domain (HEX-1.4). Byte for byte
//! the ones the characterization tests fix (`app/tests/communities_api.rs`).
//!
//! D2 a): an invalid owner keeps the `IAM_` codes. The rest of the
//! communities codes come here with the exhaustive `fold` of
//! `CreateCommunity` (R17).

/// Body of the 500 for a `Fault` (D7), until Problem Details (R30).
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
pub const INTERNAL_ERROR_MESSAGE: &str =
    "We couldn't complete your request right now. Please try again.";

pub const IAM_INVALID_ACCOUNT_ID: &str = "IAM_INVALID_ACCOUNT_ID";
pub const IAM_INVALID_ACCOUNT_ID_FORMAT: &str = "IAM_INVALID_ACCOUNT_ID_FORMAT";

//! Stable client codes and messages (ERR-4.4): transport vocabulary, so they
//! live in the HTTP adapter and not in the core. Byte for byte the ones the
//! characterization tests fix (`app/tests/communities_api.rs`).

/// Body of the 500 for a `Fault` (D7), until Problem Details (R30).
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
pub const INTERNAL_ERROR_MESSAGE: &str =
    "We couldn't complete your request right now. Please try again.";

// `POST /communities/create` (CreateCommunity).
pub const COMMUNITIES_INVALID_COMMUNITY_SLUG: &str = "COMMUNITIES_INVALID_COMMUNITY_SLUG";
pub const COMMUNITIES_INVALID_COMMUNITY_SLUG_MESSAGE: &str = "Please enter a valid community slug.";
pub const COMMUNITIES_INVALID_COMMUNITY_NAME: &str = "COMMUNITIES_INVALID_COMMUNITY_NAME";
pub const COMMUNITIES_INVALID_COMMUNITY_NAME_MESSAGE: &str = "Please enter a valid community name.";
pub const COMMUNITIES_SLUG_ALREADY_EXISTS: &str = "COMMUNITIES_SLUG_ALREADY_EXISTS";
pub const COMMUNITIES_SLUG_ALREADY_EXISTS_MESSAGE: &str = "This community slug is already in use.";

// D2 a): an invalid owner keeps the `IAM_` codes that
// `app/tests/communities_api.rs` fixes, even though this is not `iam`.
pub const IAM_INVALID_ACCOUNT_ID: &str = "IAM_INVALID_ACCOUNT_ID";
pub const IAM_INVALID_ACCOUNT_ID_MESSAGE: &str = "The account ID is invalid.";
pub const IAM_INVALID_ACCOUNT_ID_FORMAT: &str = "IAM_INVALID_ACCOUNT_ID_FORMAT";
pub const IAM_INVALID_ACCOUNT_ID_FORMAT_MESSAGE: &str = "The account ID format is invalid.";

//! Stable client codes and messages (ERR-4.4): transport vocabulary, so they
//! live in the HTTP adapter and not in the core. Byte for byte the ones the
//! characterization tests fix (`app/tests/`).

/// Body of the 500 for a `Fault` (D7), until Problem Details (R30).
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
pub const INTERNAL_ERROR_MESSAGE: &str =
    "We couldn't complete your request right now. Please try again.";

// `POST /auth/register` (RegisterAccount).
pub const IAM_PASSWORD_TOO_SHORT: &str = "IAM_PASSWORD_TOO_SHORT";
pub const IAM_PASSWORD_TOO_SHORT_MESSAGE: &str = "Password must be at least 8 characters long.";
pub const IAM_INVALID_USERNAME: &str = "IAM_INVALID_USERNAME";
pub const IAM_INVALID_USERNAME_MESSAGE: &str = "Please enter a valid username.";
pub const IAM_INVALID_EMAIL: &str = "IAM_INVALID_EMAIL";
pub const IAM_INVALID_EMAIL_MESSAGE: &str = "Please enter a valid email address.";
pub const IAM_ACCOUNT_EMAIL_ALREADY_EXISTS: &str = "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS";
pub const IAM_ACCOUNT_EMAIL_ALREADY_EXISTS_MESSAGE: &str = "This email is already in use.";
pub const IAM_ACCOUNT_USERNAME_ALREADY_EXISTS: &str = "IAM_ACCOUNT_USERNAME_ALREADY_EXISTS";
pub const IAM_ACCOUNT_USERNAME_ALREADY_EXISTS_MESSAGE: &str = "This username is already in use.";

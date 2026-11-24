//! Stable client codes and messages (ERR-4.4): transport vocabulary, so they
//! live in the HTTP adapter and not in the core. Byte for byte the ones the
//! characterization tests fix (`app/tests/`).

/// Body of the 500 for a `Fault` (D7), until Problem Details (R30).
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
pub const INTERNAL_ERROR_MESSAGE: &str =
    "We couldn't complete your request right now. Please try again.";

use shared::domain::DomainError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCodeValidation;

impl fmt::Display for InvalidCodeValidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid code validation")
    }
}

impl DomainError for InvalidCodeValidation {
    fn code(&self) -> &'static str {
        "IAM_INVALID_CODE_VALIDATION"
    }
}

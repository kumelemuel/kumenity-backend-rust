use crate::domain::errors::CodeValidationError;
use shared::ValueObject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeValidation(u32);

const MIN: u32 = 100_000;
const MAX: u32 = 999_999;

impl ValueObject for CodeValidation {}

impl CodeValidation {
    pub fn new(value: u32) -> Result<Self, CodeValidationError> {
        if (MIN..=MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(CodeValidationError::Invalid)
        }
    }

    pub fn value(&self) -> &u32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_six_digits_value() {
        assert!(CodeValidation::new(100_000).is_ok());
        assert!(CodeValidation::new(999_999).is_ok());
    }

    #[test]
    fn rejects_out_of_bounds() {
        assert_eq!(
            CodeValidation::new(99_999),
            Err(CodeValidationError::Invalid)
        );
        assert_eq!(
            CodeValidation::new(1_000_000),
            Err(CodeValidationError::Invalid)
        );
    }
}

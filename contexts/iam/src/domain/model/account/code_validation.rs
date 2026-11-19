use crate::domain::errors::CodeValidationError;
use rand::Rng;
use shared::domain::value_object::ValueObject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeValidation(u32);

const MIN: u32 = 100_000;
const MAX: u32 = 999_999;

impl ValueObject for CodeValidation {
    type Value = u32;
    type Error = CodeValidationError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        if (MIN..=MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(CodeValidationError::Invalid)
        }
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

impl CodeValidation {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let code: u32 = rng.gen_range(100_000..=999_999);
        Self(code)
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

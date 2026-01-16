use rand::Rng;
use crate::domain::errors::InvalidCodeValidation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeValidation(u32);

impl CodeValidation {
    const MIN: u32 = 100_000;
    const MAX: u32 = 999_999;

    pub fn new(value: u32) -> Result<Self, InvalidCodeValidation> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidCodeValidation)
        }
    }

    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let code: u32 = rng.gen_range(100_000..=999_999);
        Self(code)
    }

    pub fn value(&self) -> u32 {
        self.0
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
        assert_eq!(CodeValidation::new(99_999), Err(InvalidCodeValidation));
        assert_eq!(CodeValidation::new(1_000_000), Err(InvalidCodeValidation));
    }
}

use crate::domain::errors::password_policy::PasswordPolicyError;

pub struct PasswordPolicy;

impl PasswordPolicy {
    pub fn validate(raw: &str) -> Result<(), PasswordPolicyError> {
        if raw.len() < 8 {
            return Err(PasswordPolicyError::TooShort.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_password_with_minimum_length() {
        let password = "strongpw";

        let result = PasswordPolicy::validate(password);

        assert!(result.is_ok());
    }

    #[test]
    fn accepts_password_longer_than_minimum_length() {
        let password = "this_is_a_very_strong_password";

        let result = PasswordPolicy::validate(password);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_password_shorter_than_minimum_length() {
        let password = "short";

        let result = PasswordPolicy::validate(password);

        assert_eq!(result, Err(PasswordPolicyError::TooShort));
    }

    #[test]
    fn rejects_empty_password() {
        let password = "";

        let result = PasswordPolicy::validate(password);

        assert_eq!(result, Err(PasswordPolicyError::TooShort));
    }
}

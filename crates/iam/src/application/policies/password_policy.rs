use crate::application::errors::password_policy::PasswordPolicyError;
use shared::error::SystemError;

pub struct PasswordPolicy;

impl PasswordPolicy {
    pub fn validate(raw: &str) -> Result<(), SystemError> {
        if raw.len() < 8 {
            return Err(PasswordPolicyError::TooShort.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::errors::error_codes::IAM_PASSWORD_TOO_SHORT;

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

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_PASSWORD_TOO_SHORT);
    }

    #[test]
    fn rejects_empty_password() {
        let password = "";

        let result = PasswordPolicy::validate(password);

        let err = result.expect_err("Expected error");

        assert_eq!(err.code(), IAM_PASSWORD_TOO_SHORT);
    }
}

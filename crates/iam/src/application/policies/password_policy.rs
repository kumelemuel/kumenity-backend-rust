use crate::application::errors::application_error::ApplicationError;

pub struct PasswordPolicy;

impl PasswordPolicy {
    pub fn validate(raw: &str) -> Result<(), ApplicationError> {
        if raw.len() < 8 {
            return Err(ApplicationError::InvalidPassword);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::errors::application_error::ApplicationError;

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

        assert!(matches!(result, Err(ApplicationError::InvalidPassword)));
    }

    #[test]
    fn rejects_empty_password() {
        let password = "";

        let result = PasswordPolicy::validate(password);

        assert!(matches!(result, Err(ApplicationError::InvalidPassword)));
    }
}

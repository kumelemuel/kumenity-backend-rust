use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordPolicyError {
    TooShort,
}

impl fmt::Display for PasswordPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordPolicyError::TooShort => {
                write!(f, "Password must be at least 8 characters long")
            }
        }
    }
}

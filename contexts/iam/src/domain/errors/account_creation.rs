use super::{EmailError, UsernameError, password_policy::PasswordPolicyError};

/// Why `AccountFactory::draft` refused (closed, per rule): the handler
/// translates each variant to its own leaf, by variant and never by code
/// (ERR-2). The order of the checks is the factory's.
#[derive(Debug, PartialEq, Eq)]
pub enum AccountCreationError {
    Password(PasswordPolicyError),
    Username(UsernameError),
    Email(EmailError),
}

impl From<PasswordPolicyError> for AccountCreationError {
    fn from(error: PasswordPolicyError) -> Self {
        Self::Password(error)
    }
}

impl From<UsernameError> for AccountCreationError {
    fn from(error: UsernameError) -> Self {
        Self::Username(error)
    }
}

impl From<EmailError> for AccountCreationError {
    fn from(error: EmailError) -> Self {
        Self::Email(error)
    }
}

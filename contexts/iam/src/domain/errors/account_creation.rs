use super::{EmailError, HashedPasswordError, UsernameError, password_policy::PasswordPolicyError};

/// Why `AccountFactory::create` refused (closed, per rule): the handler
/// translates each variant to its own leaf or tree, by variant and never by
/// code (ERR-2). The order of the checks is the factory's.
#[derive(Debug, PartialEq, Eq)]
pub enum AccountCreationError {
    Password(PasswordPolicyError),
    Username(UsernameError),
    Email(EmailError),
    HashedPassword(HashedPasswordError),
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

impl From<HashedPasswordError> for AccountCreationError {
    fn from(error: HashedPasswordError) -> Self {
        Self::HashedPassword(error)
    }
}

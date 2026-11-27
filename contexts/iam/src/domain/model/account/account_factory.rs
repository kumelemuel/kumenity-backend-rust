use crate::domain::{
    errors::AccountCreationError,
    model::account::{
        account::Account, account_id::AccountId, account_status::AccountStatus,
        code_validation::CodeValidation, email::Email, hashed_password::HashedPassword,
        username::Username,
    },
    services::password_policy::PasswordPolicy,
};
use shared::domain::value_object::ValueObject;

/// What a registration asks for, already checked against the rules of the
/// domain: the password policy, and the username and email as normalized
/// value objects. It is the only way through `AccountFactory::create`.
/// `Account::new` is still public until R20b (HEX-7.5), and that the hash
/// comes from the password the policy accepted is up to the caller.
///
/// The raw password is not kept: once the policy accepts it, only its hash
/// goes into the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountDraft {
    username: Username,
    email: Email,
}

impl AccountDraft {
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}

#[derive(Debug, Clone)]
pub struct AccountFactory {}
impl AccountFactory {
    /// Checks the registration input in the order the contract fixes:
    /// password, username, email (`register_validates_password_before_username_and_email`).
    pub fn draft(
        username: String,
        email: String,
        raw_password: &str,
    ) -> Result<AccountDraft, AccountCreationError> {
        PasswordPolicy::validate(raw_password)?;
        let username = Username::new(username)?;
        let email = Email::new(email)?;

        Ok(AccountDraft { username, email })
    }

    /// The id comes from the `IdGenerator` (HEX-7.6): the domain does not
    /// generate it.
    pub fn create(id: AccountId, draft: AccountDraft, password: HashedPassword) -> Account {
        let status = AccountStatus::Registered {
            code_validation: CodeValidation::generate(),
        };

        Account::new(id, draft.username, draft.email, password, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::{EmailError, UsernameError, password_policy::PasswordPolicyError};

    const VALID_HASH: &str = "$argon2id$v=19$m=65536,t=2,p=1$hashhashhashhashhashhashhashhashhash";

    fn draft(
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AccountDraft, AccountCreationError> {
        AccountFactory::draft(username.into(), email.into(), password)
    }

    fn id() -> AccountId {
        AccountId::new("acc-1".to_string())
    }

    fn valid_hash() -> HashedPassword {
        HashedPassword::new(VALID_HASH.into()).expect("invariante: hash de prueba válido")
    }

    fn valid_draft() -> AccountDraft {
        draft("john_doe", "john@example.com", "password123").expect("invariante: borrador válido")
    }

    #[test]
    fn draft_rejects_short_password_with_password_variant() {
        let result = draft("john_doe", "john@example.com", "1234567");

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Password(
                PasswordPolicyError::TooShort
            ))
        );
    }

    #[test]
    fn draft_rejects_invalid_username_with_username_variant() {
        let result = draft("ab", "john@example.com", "password123");

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Username(UsernameError::Invalid))
        );
    }

    #[test]
    fn draft_rejects_invalid_email_with_email_variant() {
        let result = draft("john_doe", "not-an-email", "password123");

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Email(EmailError::Invalid))
        );
    }

    #[test]
    fn create_keeps_the_drafted_username_and_email() {
        let draft = valid_draft();

        let account = AccountFactory::create(id(), draft.clone(), valid_hash());

        assert_eq!(account.username(), draft.username());
        assert_eq!(account.email(), draft.email());
    }

    /// Order fixed by `register_validates_password_before_username_and_email`.
    #[test]
    fn draft_checks_password_then_username_then_email() {
        assert_eq!(
            draft("ab", "not-an-email", "short").err(),
            Some(AccountCreationError::Password(
                PasswordPolicyError::TooShort
            ))
        );
        assert_eq!(
            draft("ab", "not-an-email", "password123").err(),
            Some(AccountCreationError::Username(UsernameError::Invalid))
        );
        assert_eq!(
            draft("john_doe", "not-an-email", "password123").err(),
            Some(AccountCreationError::Email(EmailError::Invalid))
        );
    }

    impl AccountFactory {
        /// Pending verification with the code 123123.
        pub(crate) fn dummy_account() -> Account {
            Self::account("john_doe", "john@example.com", Self::pending(123_123))
        }

        pub fn dummy_account_with_status(status: AccountStatus) -> Account {
            Self::account("dummy", "dummy@example.com", status)
        }

        /// `Registered`, waiting for `code`.
        pub(crate) fn pending(code: u32) -> AccountStatus {
            AccountStatus::Registered {
                code_validation: CodeValidation::new(code)
                    .expect("invariante: código de prueba válido"),
            }
        }

        fn account(username: &str, email: &str, status: AccountStatus) -> Account {
            Account::restore(
                AccountId::new(format!("id-{username}")),
                Username::new(username.to_string()).expect("invariante: username de prueba válido"),
                Email::new(email.to_string()).expect("invariante: email de prueba válido"),
                HashedPassword::new(DUMMY_HASH.to_string())
                    .expect("invariante: hash de prueba válido"),
                status,
            )
        }
    }

    const DUMMY_HASH: &str =
        "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012";
}

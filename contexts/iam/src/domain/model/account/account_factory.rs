use crate::domain::{
    errors::{AccountCreationError, HashedPasswordError},
    model::account::{
        account::Account, account_id::AccountId, account_status::AccountStatus,
        code_validation::CodeValidation, email::Email, hashed_password::HashedPassword,
        username::Username,
    },
    services::password_policy::PasswordPolicy,
};
use shared::{domain::value_object::ValueObject, error::SystemError};

/// What a registration asks for, already checked against the rules of the
/// domain: the password policy, and the username and email as normalized
/// value objects. It is the only way through `AccountFactory::create`.
/// `Account::new` is still public until R20 (HEX-7.5), and that the hash
/// comes from the password the policy accepted is up to the caller (TDD-2.7).
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

    pub fn create(
        draft: AccountDraft,
        hashed_password: String,
    ) -> Result<Account, HashedPasswordError> {
        let password = HashedPassword::new(hashed_password)?;
        let status = AccountStatus::Registered {
            code_validation: CodeValidation::generate(),
        };

        Ok(Account::new(
            AccountId::generate(),
            draft.username,
            draft.email,
            password,
            status,
        ))
    }

    pub fn reconstitute(
        id: String,
        username: String,
        email: String,
        hashed_password: String,
        status: String,
    ) -> Result<Account, SystemError> {
        let id = AccountId::from_str(id.as_str())?;
        let username = Username::new(username)?;
        let email = Email::new(email)?;
        let password = HashedPassword::new(hashed_password)?;
        let status = AccountStatus::from_str(status.as_str())?;

        Ok(Account::new(id, username, email, password, status))
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
    fn create_rejects_malformed_hash() {
        let result = AccountFactory::create(valid_draft(), "short".into());

        assert_eq!(result.err(), Some(HashedPasswordError::Invalid));
    }

    #[test]
    fn create_keeps_the_drafted_username_and_email() {
        let draft = valid_draft();

        let account = AccountFactory::create(draft.clone(), VALID_HASH.into())
            .expect("invariante: hash de prueba válido");

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
        pub(crate) fn dummy_account() -> Account {
            AccountFactory::reconstitute(
                AccountId::generate().as_uuid().to_string(),
                "john_doe".to_string(),
                "john@example.com".to_string(),
                "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
                "registered:123123".to_string()
            )
                .unwrap()
        }

        pub fn dummy_account_with_status(status: String) -> Account {
            AccountFactory::reconstitute(
                AccountId::generate().as_uuid().to_string(),
                "dummy".to_string(),
                "dummy@example.com".to_string(),
                "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012".to_string(),
                status,
            ).unwrap()
        }
    }
}

use crate::domain::{
    errors::AccountCreationError,
    model::account::{
        account::Account, account_id::AccountId, account_status::AccountStatus,
        code_validation::CodeValidation, email::Email, hashed_password::HashedPassword,
        username::Username,
    },
    services::password_policy::PasswordPolicy,
};
use shared::{domain::value_object::ValueObject, error::SystemError};

#[derive(Debug, Clone)]
pub struct AccountFactory {}
impl AccountFactory {
    pub fn create(
        username: String,
        email: String,
        raw_password: &str,
        hashed_password: String,
    ) -> Result<Account, AccountCreationError> {
        PasswordPolicy::validate(raw_password)?;
        let id = AccountId::generate();
        let username = Username::new(username)?;
        let email = Email::new(email)?;
        let password = HashedPassword::new(hashed_password)?;
        let status = AccountStatus::Registered {
            code_validation: CodeValidation::generate(),
        };

        Ok(Account::new(id, username, email, password, status))
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
    use crate::domain::errors::{
        EmailError, HashedPasswordError, UsernameError, password_policy::PasswordPolicyError,
    };

    const VALID_HASH: &str = "$argon2id$v=19$m=65536,t=2,p=1$hashhashhashhashhashhashhashhashhash";

    fn create(
        username: &str,
        email: &str,
        password: &str,
        hash: &str,
    ) -> Result<Account, AccountCreationError> {
        AccountFactory::create(username.into(), email.into(), password, hash.into())
    }

    #[test]
    fn create_rejects_short_password_with_password_variant() {
        let result = create("john_doe", "john@example.com", "1234567", VALID_HASH);

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Password(
                PasswordPolicyError::TooShort
            ))
        );
    }

    #[test]
    fn create_rejects_invalid_username_with_username_variant() {
        let result = create("ab", "john@example.com", "password123", VALID_HASH);

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Username(UsernameError::Invalid))
        );
    }

    #[test]
    fn create_rejects_invalid_email_with_email_variant() {
        let result = create("john_doe", "not-an-email", "password123", VALID_HASH);

        assert_eq!(
            result.err(),
            Some(AccountCreationError::Email(EmailError::Invalid))
        );
    }

    #[test]
    fn create_rejects_malformed_hash_with_hashed_password_variant() {
        let result = create("john_doe", "john@example.com", "password123", "short");

        assert_eq!(
            result.err(),
            Some(AccountCreationError::HashedPassword(
                HashedPasswordError::Invalid
            ))
        );
    }

    /// Order fixed by `register_validates_password_before_username_and_email`.
    #[test]
    fn create_checks_password_then_username_then_email() {
        assert_eq!(
            create("ab", "not-an-email", "short", "short").err(),
            Some(AccountCreationError::Password(
                PasswordPolicyError::TooShort
            ))
        );
        assert_eq!(
            create("ab", "not-an-email", "password123", "short").err(),
            Some(AccountCreationError::Username(UsernameError::Invalid))
        );
        assert_eq!(
            create("john_doe", "not-an-email", "password123", "short").err(),
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

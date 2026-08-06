use crate::domain::{
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
    ) -> Result<Account, SystemError> {
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

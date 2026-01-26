use crate::domain::errors::InvalidAccountStatusTransition;
use crate::domain::value_objects::account_id::AccountId;
use crate::domain::value_objects::{CodeValidation, Email, HashedPassword, AccountStatus, Username};

#[derive(Debug, Clone)]
pub struct Account {
    id: AccountId,
    username: Username,
    email: Email,
    password: HashedPassword,
    status: AccountStatus,
}

impl Account {
    pub fn register(
        id: AccountId,
        username: Username,
        email: Email,
        password: HashedPassword,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
            status: AccountStatus::Registered{ code_validation: CodeValidation::generate() },
        }
    }

    pub fn reconstitute(
        id: AccountId,
        username: Username,
        email: Email,
        password: HashedPassword,
        status: AccountStatus,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
            status,
        }
    }

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn status(&self) -> &AccountStatus {
        &self.status
    }

    pub fn can_authenticate(&self) -> bool {
        self.status.can_authenticate()
    }

    pub fn change_password(&mut self, new_password: HashedPassword) {
        self.password = new_password;
    }

    pub fn change_email(&mut self, new_email: Email) {
        self.email = new_email;
    }

    pub fn change_username(&mut self, new_username: Username) {
        self.username = new_username;
    }

    fn transition_status(&mut self, next: AccountStatus) -> Result<(), InvalidAccountStatusTransition> {
        self.status = self.status.transition_to(next)?;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), InvalidAccountStatusTransition> {
        self.transition_status(AccountStatus::Deactivated)
    }

    pub fn activate(&mut self) -> Result<(), InvalidAccountStatusTransition> {
        self.transition_status(AccountStatus::Active)
    }

    pub fn suspend(&mut self) -> Result<(), InvalidAccountStatusTransition> {
        self.transition_status(AccountStatus::Suspended)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{
        email::Email, hashed_password::HashedPassword, account_id::AccountId, username::Username,
    };

    fn registered_user() -> Account {
        Account::register(
            AccountId::generate(),
            Username::new("john_doe".to_string()).unwrap(),
            Email::new("john@example.com").unwrap(),
            HashedPassword::dummy(),
        )
    }

    #[test]
    fn registering_user_starts_in_registered_status() {
        let user = registered_user();

        assert!(matches!(
            user.status(),
            AccountStatus::Registered { .. }
        ));
    }

    #[test]
    fn registered_user_cannot_be_deactivated() {
        let mut user = registered_user();

        let result = user.deactivate();

        assert!(result.is_err());
    }

    #[test]
    fn allows_changing_password() {
        let mut user = registered_user();
        let new_password = HashedPassword::from_hash("x".repeat(60)).unwrap();

        user.change_password(new_password.clone());

        assert_eq!(user.password(), &new_password);
    }

    #[test]
    fn deleted_user_cannot_be_deactivated() {
        let mut user = Account::reconstitute(
            AccountId::generate(),
            Username::new("john".into()).unwrap(),
            Email::new("john@example.com").unwrap(),
            HashedPassword::dummy(),
            AccountStatus::Deleted,
        );

        assert!(user.deactivate().is_err());
    }

    #[test]
    fn registered_user_can_be_activated() {
        let mut user = registered_user();

        assert!(user.activate().is_ok());
        assert_eq!(user.status(), &AccountStatus::Active);
    }

    #[test]
    fn registered_user_cannot_be_suspended() {
        let mut user = registered_user();

        assert!(user.suspend().is_err());
    }

    #[test]
    fn reconstituted_user_preserves_status() {
        let user = Account::reconstitute(
            AccountId::generate(),
            Username::new("john".into()).unwrap(),
            Email::new("john@example.com").unwrap(),
            HashedPassword::dummy(),
            AccountStatus::Suspended,
        );

        assert_eq!(user.status(), &AccountStatus::Suspended);
    }
}

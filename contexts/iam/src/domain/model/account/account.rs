use crate::domain::{
    errors::{AccountError, AccountStatusError},
    model::account::{
        account_id::AccountId, account_status::AccountStatus, code_validation::CodeValidation,
        email::Email, events::account_event::AccountEvent, hashed_password::HashedPassword,
        username::Username,
    },
};
use shared::domain::{
    aggregate_root::{AggregateRoot, EventRecorder},
    entity::Entity,
};

#[derive(Debug, Clone)]
pub struct Account {
    id: AccountId,
    username: Username,
    email: Email,
    password: HashedPassword,
    status: AccountStatus,
    _events: EventRecorder<AccountEvent>,
}

impl Entity for Account {
    type Id = AccountId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl AggregateRoot for Account {
    type Event = AccountEvent;

    fn record_event(&mut self, _event: Self::Event) {
        todo!()
    }

    fn pull_events(&mut self) -> Vec<Self::Event> {
        todo!()
    }

    fn version(&self) -> u64 {
        todo!()
    }
}

impl Account {
    pub fn new(
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
            _events: EventRecorder::default(),
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

    fn transition_status(&mut self, next: AccountStatus) -> Result<(), AccountStatusError> {
        self.status = self.status.transition_to(next)?;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), AccountStatusError> {
        self.transition_status(AccountStatus::Deactivated)
    }

    pub fn activate(&mut self) -> Result<(), AccountStatusError> {
        if self.status.as_str() == "registered" {
            return Err(AccountStatusError::InvalidTransition);
        }
        self.transition_status(AccountStatus::Active)
    }

    pub fn suspend(&mut self) -> Result<(), AccountStatusError> {
        self.transition_status(AccountStatus::Suspended)
    }

    pub fn confirm_registration(&mut self, code: CodeValidation) -> Result<(), AccountError> {
        match self.status {
            AccountStatus::Registered { code_validation } => {
                if code_validation != code {
                    return Err(AccountError::InvalidVerification);
                }
                let _ = self
                    .transition_status(AccountStatus::Active)
                    .map_err(|_| AccountError::InvalidVerification);
                Ok(())
            }
            _ => Err(AccountError::InvalidVerification),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::account::account_factory::AccountFactory;
    use shared::domain::value_object::ValueObject;
    #[test]
    fn registering_user_starts_in_registered_status() {
        let user = AccountFactory::dummy_account();

        assert!(matches!(user.status(), AccountStatus::Registered { .. }));
    }

    #[test]
    fn registered_user_cannot_be_deactivated() {
        let mut user = AccountFactory::dummy_account();

        let result = user.deactivate();

        assert!(result.is_err());
    }

    #[test]
    fn allows_changing_password() {
        let mut user = AccountFactory::dummy_account();
        let new_password = HashedPassword::new("x".repeat(60)).unwrap();

        user.change_password(new_password.clone());

        assert_eq!(user.password(), &new_password);
    }

    #[test]
    fn deleted_user_cannot_be_deactivated() {
        let mut user = AccountFactory::dummy_account_with_status("deleted".to_string());

        assert!(user.deactivate().is_err());
    }

    #[test]
    fn registered_user_cannot_be_activated_directly() {
        let mut user = AccountFactory::dummy_account();

        assert!(user.activate().is_err());
    }

    #[test]
    fn should_not_allow_confirmation_from_non_registered_states() {
        let states = vec![
            AccountStatus::Active,
            AccountStatus::Suspended,
            AccountStatus::Deactivated,
            AccountStatus::Deleted,
        ];

        let code = CodeValidation::new(123123).unwrap();

        for state in states {
            let mut user = AccountFactory::dummy_account_with_status(state.as_str().to_string());
            let result = user.confirm_registration(code);
            assert!(result.is_err());
        }
    }

    #[test]
    fn registered_user_can_be_confirmed_with_correct_code_validation() {
        let mut user = AccountFactory::dummy_account_with_status("registered:123123".to_string());

        let code = CodeValidation::new(123123).unwrap();

        assert!(user.confirm_registration(code).is_ok());
    }

    #[test]
    fn registered_user_cannot_be_confirmed_with_invalid_code_validation() {
        let mut user = AccountFactory::dummy_account();

        let code = CodeValidation::new(321321).unwrap();

        assert!(user.confirm_registration(code).is_err());
    }

    #[test]
    fn registered_user_cannot_be_suspended() {
        let mut user = AccountFactory::dummy_account();

        assert!(user.suspend().is_err());
    }

    #[test]
    fn reconstituted_user_preserves_status() {
        let user = AccountFactory::dummy_account_with_status("suspended".to_string());

        assert_eq!(user.status(), &AccountStatus::Suspended);
    }
}

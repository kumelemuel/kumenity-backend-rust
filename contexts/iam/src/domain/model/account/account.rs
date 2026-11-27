use crate::domain::{
    errors::{AccountError, AccountStatusError, CannotAuthenticate},
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

    /// An account as it was saved: the adapter rebuilds it from its value
    /// objects, without the rules of a registration (HEX-7.5).
    pub fn restore(
        id: AccountId,
        username: Username,
        email: Email,
        password: HashedPassword,
        status: AccountStatus,
    ) -> Self {
        Self::new(id, username, email, password, status)
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

    /// Only an active account signs in (HEX-7.2).
    pub fn authenticate(&self) -> Result<(), CannotAuthenticate> {
        if self.status.can_authenticate() {
            Ok(())
        } else {
            Err(CannotAuthenticate)
        }
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
                // `can_transition_to` always admits Registered → Active: a
                // failure here is a bug, not a client error (ERR-1.4).
                self.transition_status(AccountStatus::Active)
                    .expect("invariante: Registered→Active siempre es válida");
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
    fn registered_account_cannot_authenticate() {
        let account = AccountFactory::dummy_account();

        assert_eq!(account.authenticate(), Err(CannotAuthenticate));
    }

    #[test]
    fn non_active_accounts_cannot_authenticate() {
        for status in [
            AccountStatus::Suspended,
            AccountStatus::Deactivated,
            AccountStatus::Deleted,
        ] {
            let account = AccountFactory::dummy_account_with_status(status);

            assert_eq!(
                account.authenticate(),
                Err(CannotAuthenticate),
                "{status:?}"
            );
        }
    }

    #[test]
    fn active_account_can_authenticate() {
        let account = AccountFactory::dummy_account_with_status(AccountStatus::Active);

        assert_eq!(account.authenticate(), Ok(()));
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
        let mut user = AccountFactory::dummy_account_with_status(AccountStatus::Deleted);

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
            let mut user = AccountFactory::dummy_account_with_status(state);
            let result = user.confirm_registration(code);
            assert!(result.is_err());
        }
    }

    #[test]
    fn registered_user_can_be_confirmed_with_correct_code_validation() {
        let mut user = AccountFactory::dummy_account_with_status(AccountFactory::pending(123_123));

        let code = CodeValidation::new(123123).unwrap();

        assert!(user.confirm_registration(code).is_ok());
    }

    #[test]
    fn confirmed_registration_leaves_the_account_active() {
        let mut user = AccountFactory::dummy_account();

        user.confirm_registration(CodeValidation::new(123123).expect("invariante: código válido"))
            .expect("confirm with the delivered code");

        assert_eq!(*user.status(), AccountStatus::Active);
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
    fn restored_account_preserves_status() {
        let account = Account::restore(
            AccountId::new("acc-1".to_string()),
            Username::new("john_doe".to_string()).expect("invariante: username de prueba válido"),
            Email::new("john@example.com".to_string()).expect("invariante: email de prueba válido"),
            HashedPassword::new("x".repeat(60)).expect("invariante: hash de prueba válido"),
            AccountStatus::Suspended,
        );

        assert_eq!(account.status(), &AccountStatus::Suspended);
    }
}

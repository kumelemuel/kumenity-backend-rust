use crate::domain::{
    errors::{AccountError, AccountStatusTransitionError},
    value_objects::{AccountId, AccountStatus, CodeValidation, Email, HashedPassword, Username},
};

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
            status: AccountStatus::Registered {
                code_validation: CodeValidation::generate(),
            },
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

    fn transition_status(
        &mut self,
        next: AccountStatus,
    ) -> Result<(), AccountStatusTransitionError> {
        self.status = self.status.transition_to(next)?;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), AccountStatusTransitionError> {
        self.transition_status(AccountStatus::Deactivated)
    }

    pub fn activate(&mut self) -> Result<(), AccountStatusTransitionError> {
        if self.status.as_str() == "registered" {
            return Err(AccountStatusTransitionError::Invalid);
        }
        self.transition_status(AccountStatus::Active)
    }

    pub fn suspend(&mut self) -> Result<(), AccountStatusTransitionError> {
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

    impl Account {
        pub fn dummy_account() -> Account {
            Account::reconstitute(
                AccountId::generate(),
                Username::new("dummy".to_string()).unwrap(),
                Email::new("dummy@example.com").unwrap(),
                HashedPassword::dummy(),
                AccountStatus::Registered {
                    code_validation: CodeValidation::new(123123).unwrap(),
                },
            )
        }

        pub fn dummy_account_with_status(status: AccountStatus) -> Account {
            Account::reconstitute(
                AccountId::generate(),
                Username::new("dummy".to_string()).unwrap(),
                Email::new("dummy@example.com").unwrap(),
                HashedPassword::dummy(),
                status,
            )
        }
    }

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

        assert!(matches!(user.status(), AccountStatus::Registered { .. }));
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
    fn registered_user_cannot_be_activated_directly() {
        let mut user = registered_user();

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
            let mut user = Account::dummy_account_with_status(state);
            let result = user.confirm_registration(code);
            assert!(result.is_err());
        }
    }

    #[test]
    fn registered_user_can_be_confirmed_with_correct_code_validation() {
        let mut user = Account::dummy_account();

        let code = CodeValidation::new(123123).unwrap();

        assert!(user.confirm_registration(code).is_ok());
    }

    #[test]
    fn registered_user_cannot_be_confirmed_with_invalid_code_validation() {
        let mut user = Account::dummy_account();

        let code = CodeValidation::new(321321).unwrap();

        assert!(user.confirm_registration(code).is_err());
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

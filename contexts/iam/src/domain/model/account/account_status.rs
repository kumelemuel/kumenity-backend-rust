use crate::domain::{errors::AccountStatusError, model::account::code_validation::CodeValidation};
use shared::domain::value_object::ValueObject;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Registered { code_validation: CodeValidation },
    Active,
    Suspended,
    Deactivated,
    Deleted,
}

impl ValueObject for AccountStatus {
    type Value = AccountStatus;
    type Error = AccountStatusError;

    fn new(_value: Self::Value) -> Result<Self, Self::Error> {
        todo!()
    }

    fn value(&self) -> &Self::Value {
        todo!()
    }
}

impl AccountStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, AccountStatus::Deleted)
    }

    pub fn can_authenticate(&self) -> bool {
        matches!(self, AccountStatus::Active)
    }

    pub fn can_transition_to(&self, next: AccountStatus) -> bool {
        use AccountStatus::*;

        matches!(
            (self, next),
            (Registered { .. }, Active)
                | (Registered { .. }, Deleted)
                | (Active, Suspended)
                | (Active, Deactivated)
                | (Active, Deleted)
                | (Suspended, Active)
                | (Deactivated, Active)
                | (Deactivated, Suspended)
                | (Deactivated, Deleted)
        )
    }

    pub fn transition_to(&self, next: AccountStatus) -> Result<AccountStatus, AccountStatusError> {
        if self.is_terminal() {
            return Err(AccountStatusError::InvalidTransition);
        }

        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(AccountStatusError::InvalidTransition)
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AccountStatus::Registered { .. } => "registered",
            AccountStatus::Active => "active",
            AccountStatus::Suspended => "suspended",
            AccountStatus::Deactivated => "deactivated",
            AccountStatus::Deleted => "deleted",
        }
    }

    pub fn from_str(value: &str) -> Result<Self, AccountStatusError> {
        let registered = value.starts_with("registered:");
        if registered {
            Ok(AccountStatus::Registered {
                code_validation: CodeValidation::new(
                    value.strip_prefix("registered:").unwrap().parse().unwrap(),
                )
                .unwrap(),
            })
        } else {
            match value {
                "active" => Ok(AccountStatus::Active),
                "suspended" => Ok(AccountStatus::Suspended),
                "deactivated" => Ok(AccountStatus::Deactivated),
                "deleted" => Ok(AccountStatus::Deleted),
                _ => Err(AccountStatusError::InvalidTransition),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_can_transition_to_active() {
        let code_validation = CodeValidation::generate();
        let status = AccountStatus::Registered { code_validation };
        let next = status.transition_to(AccountStatus::Active);

        assert_eq!(next.unwrap(), AccountStatus::Active);
    }

    #[test]
    fn active_can_transition_to_suspended() {
        let status = AccountStatus::Active;
        let next = status.transition_to(AccountStatus::Suspended);

        assert_eq!(next.unwrap(), AccountStatus::Suspended);
    }

    #[test]
    fn suspended_can_transition_back_to_active() {
        let status = AccountStatus::Suspended;
        let next = status.transition_to(AccountStatus::Active);

        assert_eq!(next.unwrap(), AccountStatus::Active);
    }

    #[test]
    fn deleted_is_terminal() {
        let status = AccountStatus::Deleted;

        assert!(status.is_terminal());
        assert_eq!(
            status.transition_to(AccountStatus::Active),
            Err(AccountStatusError::InvalidTransition)
        );
    }

    #[test]
    fn cannot_transition_from_registered_to_suspended() {
        let code_validation = CodeValidation::generate();
        let status = AccountStatus::Registered { code_validation };

        let result = status.transition_to(AccountStatus::Suspended);

        assert_eq!(result, Err(AccountStatusError::InvalidTransition));
    }

    #[test]
    fn only_active_user_can_authenticate() {
        let code_validation = CodeValidation::generate();
        assert!(AccountStatus::Active.can_authenticate());
        assert!(!AccountStatus::Registered { code_validation }.can_authenticate());
        assert!(!AccountStatus::Suspended.can_authenticate());
        assert!(!AccountStatus::Deactivated.can_authenticate());
        assert!(!AccountStatus::Deleted.can_authenticate());
    }

    #[test]
    fn deleted_cannot_transition_to_any_state() {
        let status = AccountStatus::Deleted;

        assert_eq!(
            status.transition_to(AccountStatus::Active),
            Err(AccountStatusError::InvalidTransition)
        );
    }
}

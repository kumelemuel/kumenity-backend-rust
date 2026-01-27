use crate::domain::errors::InvalidAccountStatusTransition;
use crate::domain::value_objects::CodeValidation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Registered { code_validation: CodeValidation },
    Active,
    Suspended,
    Deactivated,
    Deleted,
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
                | (Suspended, Deactivated)

                | (Deactivated, Active)
                | (Deactivated, Deleted)
        )
    }

    pub fn transition_to(
        &self,
        next: AccountStatus,
    ) -> Result<AccountStatus, InvalidAccountStatusTransition> {
        if self.is_terminal() {
            return Err(InvalidAccountStatusTransition);
        }

        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(InvalidAccountStatusTransition)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::InvalidAccountStatusTransition;

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
            Err(InvalidAccountStatusTransition)
        );
    }

    #[test]
    fn cannot_transition_from_registered_to_suspended() {
        let code_validation = CodeValidation::generate();
        let status = AccountStatus::Registered { code_validation };

        let result = status.transition_to(AccountStatus::Suspended);

        assert_eq!(result, Err(InvalidAccountStatusTransition));
    }

    #[test]
    fn only_active_user_can_authenticate() {
        let code_validation = CodeValidation::generate();
        assert!(AccountStatus::Active.can_authenticate());
        assert!(!AccountStatus::Registered{ code_validation }.can_authenticate());
        assert!(!AccountStatus::Suspended.can_authenticate());
        assert!(!AccountStatus::Deactivated.can_authenticate());
        assert!(!AccountStatus::Deleted.can_authenticate());
    }

    #[test]
    fn deleted_cannot_transition_to_any_state() {
        let status = AccountStatus::Deleted;

        assert_eq!(
            status.transition_to(AccountStatus::Active),
            Err(InvalidAccountStatusTransition)
        );
    }
}

use crate::domain::errors::InvalidUserStatusTransition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Registered,
    Active,
    Suspended,
    Deactivated,
    Deleted,
}

impl UserStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, UserStatus::Deleted)
    }

    pub fn can_authenticate(&self) -> bool {
        matches!(self, UserStatus::Active)
    }

    pub fn can_transition_to(&self, next: UserStatus) -> bool {
        use UserStatus::*;

        match (*self, next) {
            (Registered, Active) => true,
            (Registered, Deleted) => true,

            (Active, Suspended) => true,
            (Active, Deactivated) => true,
            (Active, Deleted) => true,

            (Suspended, Active) => true,
            (Suspended, Deactivated) => true,

            (Deactivated, Active) => true,
            (Deactivated, Deleted) => true,

            _ => false,
        }
    }

    pub fn transition_to(
        &self,
        next: UserStatus,
    ) -> Result<UserStatus, InvalidUserStatusTransition> {
        if self.is_terminal() {
            return Err(InvalidUserStatusTransition);
        }

        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(InvalidUserStatusTransition)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::InvalidUserStatusTransition;

    #[test]
    fn registered_can_transition_to_active() {
        let status = UserStatus::Registered;
        let next = status.transition_to(UserStatus::Active);

        assert_eq!(next.unwrap(), UserStatus::Active);
    }

    #[test]
    fn active_can_transition_to_suspended() {
        let status = UserStatus::Active;
        let next = status.transition_to(UserStatus::Suspended);

        assert_eq!(next.unwrap(), UserStatus::Suspended);
    }

    #[test]
    fn suspended_can_transition_back_to_active() {
        let status = UserStatus::Suspended;
        let next = status.transition_to(UserStatus::Active);

        assert_eq!(next.unwrap(), UserStatus::Active);
    }

    #[test]
    fn deleted_is_terminal() {
        let status = UserStatus::Deleted;

        assert!(status.is_terminal());
        assert_eq!(
            status.transition_to(UserStatus::Active),
            Err(InvalidUserStatusTransition)
        );
    }

    #[test]
    fn cannot_transition_from_registered_to_suspended() {
        let status = UserStatus::Registered;

        let result = status.transition_to(UserStatus::Suspended);

        assert_eq!(result, Err(InvalidUserStatusTransition));
    }

    #[test]
    fn only_active_user_can_authenticate() {
        assert!(UserStatus::Active.can_authenticate());
        assert!(!UserStatus::Registered.can_authenticate());
        assert!(!UserStatus::Suspended.can_authenticate());
        assert!(!UserStatus::Deactivated.can_authenticate());
        assert!(!UserStatus::Deleted.can_authenticate());
    }

    #[test]
    fn deleted_cannot_transition_to_any_state() {
        let status = UserStatus::Deleted;

        assert_eq!(
            status.transition_to(UserStatus::Active),
            Err(InvalidUserStatusTransition)
        );
    }
}

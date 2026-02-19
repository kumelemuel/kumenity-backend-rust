use crate::domain::errors::invalid_membership_status_transition::InvalidMembershipStatusTransition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipStatus {
    Pending,
    Active,
    Suspended,
    Banned,
}

impl MembershipStatus {
    pub fn can_interact(&self) -> bool {
        matches!(self, MembershipStatus::Active)
    }

    pub fn can_transition_to(&self, next: MembershipStatus) -> bool {
        use MembershipStatus::*;

        matches!(
            (self, next),
            (Pending, Active)
            | (Active, Suspended)
            | (Active, Banned)
            | (Suspended, Active)
            | (Suspended, Banned)
        )
    }

    pub fn transition_to(
        &self,
        next: MembershipStatus,
    ) -> Result<MembershipStatus, InvalidMembershipStatusTransition> {
        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(InvalidMembershipStatusTransition)
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipStatus::Pending => "pending",
            MembershipStatus::Active => "active",
            MembershipStatus::Suspended => "suspended",
            MembershipStatus::Banned => "banned",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_can_transition_to_active() {
        let status = MembershipStatus::Pending;
        let next = status.transition_to(MembershipStatus::Active);

        assert_eq!(next.unwrap(), MembershipStatus::Active);
    }

    #[test]
    fn active_can_transition_to_suspended() {
        let status = MembershipStatus::Active;
        let next = status.transition_to(MembershipStatus::Suspended);

        assert_eq!(next.unwrap(), MembershipStatus::Suspended);
    }

    #[test]
    fn active_can_transition_to_banned() {
        let status = MembershipStatus::Active;
        let next = status.transition_to(MembershipStatus::Banned);

        assert_eq!(next.unwrap(), MembershipStatus::Banned);
    }

    #[test]
    fn suspended_can_transition_back_to_active() {
        let status = MembershipStatus::Suspended;
        let next = status.transition_to(MembershipStatus::Active);

        assert_eq!(next.unwrap(), MembershipStatus::Active);
    }

    #[test]
    fn suspended_can_transition_to_banned() {
        let status = MembershipStatus::Suspended;
        let next = status.transition_to(MembershipStatus::Banned);

        assert_eq!(next.unwrap(), MembershipStatus::Banned);
    }

    #[test]
    fn cannot_transition_from_pending_to_suspended() {
        let status = MembershipStatus::Pending;

        let result = status.transition_to(MembershipStatus::Suspended);

        assert_eq!(result, Err(InvalidMembershipStatusTransition));
    }

    #[test]
    fn only_active_user_can_interact() {
        assert!(MembershipStatus::Active.can_interact());
        assert!(!MembershipStatus::Pending.can_interact());
        assert!(!MembershipStatus::Suspended.can_interact());
        assert!(!MembershipStatus::Banned.can_interact());
    }

    #[test]
    fn banned_cannot_transition_to_any_state() {
        let status = MembershipStatus::Banned;

        assert_eq!(
            status.transition_to(MembershipStatus::Active),
            Err(InvalidMembershipStatusTransition)
        );
    }
}

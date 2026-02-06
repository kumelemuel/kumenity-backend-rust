use crate::domain::errors::invalid_membership_status_transition::InvalidMembershipStatusTransition;
use crate::domain::errors::membership_error::MembershipError;
use crate::domain::value_objects::membership_status::MembershipStatus;
use crate::domain::value_objects::nickname::Nickname;
use crate::domain::value_objects::role::Role;

#[derive(Debug, PartialEq, Clone)]
pub struct Membership {
    pub role: Role,
    pub nickname: Option<Nickname>,
    pub status: MembershipStatus,
}

impl Membership {
    pub fn owner(nickname: Option<Nickname>) -> Self {
        Self {
            role: Role::Owner,
            nickname,
            status: MembershipStatus::Active,
        }
    }

    pub fn member(role: Role, nickname: Option<Nickname>) -> Result<Self, MembershipError> {
        if role == Role::Owner {
            return Err(MembershipError::CannotChangeOwnerRole);
        }

        Ok(Self {
            role,
            nickname,
            status: MembershipStatus::Pending,
        })
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn nickname(&self) -> Option<&Nickname> {
        self.nickname.as_ref()
    }

    pub fn status(&self) -> MembershipStatus {
        self.status
    }

    pub fn change_role(&mut self, new_role: Role) -> Result<(), MembershipError> {
        if self.role == Role::Owner {
            return Err(MembershipError::CannotChangeOwnerRole);
        }

        self.role = new_role;
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), MembershipError> {
        self.transition_status(MembershipStatus::Active).map_err(|_| MembershipError::InvalidStatusTransition)
    }

    pub fn suspend(&mut self) -> Result<(), MembershipError> {
        self.transition_status(MembershipStatus::Suspended).map_err(|_| MembershipError::InvalidStatusTransition)
    }

    pub fn ban(&mut self) -> Result<(), MembershipError> {
        self.transition_status(MembershipStatus::Banned).map_err(|_| MembershipError::InvalidStatusTransition)
    }

    pub fn change_nickname(
        &mut self,
        nickname: Option<Nickname>,
    ) -> Result<(), MembershipError> {
        if self.status != MembershipStatus::Active {
            return Err(MembershipError::InactiveMembership);
        }

        self.nickname = nickname;
        Ok(())
    }

    fn transition_status(&mut self, next: MembershipStatus) -> Result<(), InvalidMembershipStatusTransition> {
        self.status = self.status.transition_to(next)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_membership_is_always_active() {
        let membership = Membership::owner(None);

        assert_eq!(membership.role(), Role::Owner);
        assert_eq!(membership.status(), MembershipStatus::Active);
    }

    #[test]
    fn owner_can_have_nickname() {
        let nickname = Nickname::new("owner_nick".to_string()).unwrap();
        let membership = Membership::owner(Some(nickname.clone()));

        assert_eq!(membership.nickname().unwrap().as_str(), nickname.as_str());
    }

    #[test]
    fn member_constructor_rejects_owner_role() {
        let result = Membership::member(Role::Owner, None);

        assert_eq!(result, Err(MembershipError::CannotChangeOwnerRole));
    }

    #[test]
    fn member_starts_pending() {
        let membership = Membership::member(Role::Member, None).unwrap();

        assert_eq!(membership.role(), Role::Member);
        assert_eq!(membership.status(), MembershipStatus::Pending);
    }

    #[test]
    fn member_can_change_role() {
        let mut membership = Membership::member(Role::Member, None).unwrap();

        membership.change_role(Role::Admin).unwrap();

        assert_eq!(membership.role(), Role::Admin);
    }

    #[test]
    fn owner_cannot_change_role() {
        let mut membership = Membership::owner(None);

        let result = membership.change_role(Role::Admin);

        assert_eq!(result, Err(MembershipError::CannotChangeOwnerRole));
    }

    #[test]
    fn pending_member_can_be_activated() {
        let mut membership = Membership::member(Role::Member, None).unwrap();

        membership.activate().unwrap();

        assert_eq!(membership.status(), MembershipStatus::Active);
    }

    #[test]
    fn active_member_cannot_be_activated_again() {
        let mut membership = Membership::member(Role::Member, None).unwrap();
        membership.activate().unwrap();

        let result = membership.activate();

        assert_eq!(result, Err(MembershipError::InvalidStatusTransition));
    }

    #[test]
    fn active_member_can_be_suspended() {
        let mut membership = Membership::member(Role::Member, None).unwrap();
        membership.activate().unwrap();

        membership.suspend().unwrap();

        assert_eq!(membership.status(), MembershipStatus::Suspended);
    }

    #[test]
    fn pending_member_cannot_be_suspended() {
        let mut membership = Membership::member(Role::Member, None).unwrap();

        let result = membership.suspend();

        assert_eq!(result, Err(MembershipError::InvalidStatusTransition));
    }

    #[test]
    fn inactive_member_cannot_change_nickname() {
        let mut membership = Membership::member(Role::Member, None).unwrap();
        let nickname = Nickname::new("newnick".to_string()).unwrap();

        let result = membership.change_nickname(Some(nickname));

        assert_eq!(result, Err(MembershipError::InactiveMembership));
    }

    #[test]
    fn active_member_can_change_nickname() {
        let mut membership = Membership::member(Role::Member, None).unwrap();
        membership.activate().unwrap();

        let nickname = Nickname::new("newnick".to_string()).unwrap();
        membership.change_nickname(Some(nickname.clone())).unwrap();

        assert_eq!(membership.nickname().unwrap().as_str(), nickname.as_str());
    }

    #[test]
    fn active_member_can_remove_nickname() {
        let mut membership = Membership::member(Role::Member, None).unwrap();
        membership.activate().unwrap();

        membership.change_nickname(None).unwrap();

        assert!(membership.nickname().is_none());
    }
}

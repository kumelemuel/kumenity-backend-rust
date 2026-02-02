use std::collections::HashMap;
use iam::domain::value_objects::AccountId;
use crate::domain::entities::membership::Membership;
use crate::domain::errors::community_error::CommunityError;
use crate::domain::value_objects::community_id::CommunityId;
use crate::domain::value_objects::community_name::CommunityName;
use crate::domain::value_objects::nickname::Nickname;
use crate::domain::value_objects::role::Role;

pub struct Community {
    id: CommunityId,
    owner_id: AccountId,
    name: CommunityName,
    public: bool,
    memberships: HashMap<AccountId,Membership>
}

impl Community {
    pub fn create(
        id: CommunityId,
        owner_id: AccountId,
        name: CommunityName,
        public: bool,
        owner_nickname: Option<Nickname>,
    ) -> Self {
        let mut memberships = HashMap::new();

        memberships.insert(
            owner_id.clone(),
            Membership::owner(owner_nickname),
        );

        Self {
            id,
            owner_id,
            name,
            public,
            memberships,
        }
    }

    pub fn id(&self) -> &CommunityId {
        &self.id
    }

    pub fn name(&self) -> &CommunityName {
        &self.name
    }

    pub fn is_public(&self) -> bool {
        self.public
    }

    pub fn is_member(&self, account_id: &AccountId) -> bool {
        self.memberships.contains_key(account_id)
    }

    pub fn member(&self, account_id: &AccountId) -> Option<&Membership> {
        self.memberships.get(account_id)
    }

    pub fn add_member(
        &mut self,
        actor: &AccountId,
        new_member: AccountId,
        role: Role,
        nickname: Option<Nickname>,
    ) -> Result<(), CommunityError> {
        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_manage_members() {
            return Err(CommunityError::InsufficientPermissions);
        }

        if self.memberships.contains_key(&new_member) {
            return Err(CommunityError::AlreadyMember);
        }

        let membership =
            Membership::member(role, nickname).map_err(|_| CommunityError::InsufficientPermissions)?;

        self.memberships.insert(new_member, membership);
        Ok(())
    }

    pub fn activate_member(
        &mut self,
        actor: &AccountId,
        target: &AccountId,
    ) -> Result<(), CommunityError> {
        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_manage_members() {
            return Err(CommunityError::InsufficientPermissions);
        }

        let membership = self
            .memberships
            .get_mut(target)
            .ok_or(CommunityError::NotMember)?;

        membership.activate().map_err(|_| CommunityError::InsufficientPermissions)?;
        Ok(())
    }
    pub fn change_member_role(
        &mut self,
        actor: &AccountId,
        target: &AccountId,
        new_role: Role,
    ) -> Result<(), CommunityError> {
        if target == &self.owner_id {
            return Err(CommunityError::CannotRemoveOwner);
        }

        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_change_roles() {
            return Err(CommunityError::InsufficientPermissions);
        }

        let membership = self
            .memberships
            .get_mut(target)
            .ok_or(CommunityError::NotMember)?;

        membership
            .change_role(new_role)
            .map_err(|_| CommunityError::InsufficientPermissions)?;

        Ok(())
    }
    pub fn remove_member(
        &mut self,
        actor: &AccountId,
        target: &AccountId,
    ) -> Result<(), CommunityError> {
        if target == &self.owner_id {
            return Err(CommunityError::CannotRemoveOwner);
        }

        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_manage_members() {
            return Err(CommunityError::InsufficientPermissions);
        }

        self.memberships
            .remove(target)
            .ok_or(CommunityError::NotMember)?;

        Ok(())
    }

    pub fn suspend_member(
        &mut self,
        actor: &AccountId,
        target: &AccountId,
    ) -> Result<(), CommunityError> {
        if target == &self.owner_id {
            return Err(CommunityError::CannotRemoveOwner);
        }

        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_manage_members() {
            return Err(CommunityError::InsufficientPermissions);
        }

        let membership = self
            .memberships
            .get_mut(target)
            .ok_or(CommunityError::NotMember)?;

        membership
            .suspend()
            .map_err(|_| CommunityError::InvalidState)?;

        Ok(())
    }

    pub fn ban_member(
        &mut self,
        actor: &AccountId,
        target: &AccountId,
    ) -> Result<(), CommunityError> {
        if target == &self.owner_id {
            return Err(CommunityError::CannotRemoveOwner);
        }

        let actor_membership = self
            .memberships
            .get(actor)
            .ok_or(CommunityError::NotMember)?;

        if !actor_membership.role().can_manage_members() {
            return Err(CommunityError::InsufficientPermissions);
        }

        let membership = self
            .memberships
            .get_mut(target)
            .ok_or(CommunityError::NotMember)?;

        membership
            .ban()
            .map_err(|_| CommunityError::InvalidState)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::value_objects::membership_status::MembershipStatus;
    use super::*;

    fn community() -> Community {
        Community::create(
            CommunityId::generate(),
            AccountId::generate(),
            CommunityName::new("rust-community".to_string()).unwrap(),
            true,
            None,
        )
    }

    #[test]
    fn community_is_created_with_active_owner() {
        let community = community();

        let owner = community.member(&community.owner_id).unwrap();

        assert_eq!(owner.role(), Role::Owner);
        assert_eq!(owner.status(), MembershipStatus::Active);
    }

    #[test]
    fn owner_can_add_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        let result = community.add_member(
            &owner_id,
            member_id.clone(),
            Role::Member,
            None,
        );

        assert!(result.is_ok());
        assert!(community.is_member(&member_id));
    }

    #[test]
    fn non_admin_cannot_add_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();
        let another_member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();

        let result = community.add_member(
            &member_id,
            another_member_id,
            Role::Member,
            None,
        );

        assert_eq!(result, Err(CommunityError::InsufficientPermissions));
    }

    #[test]
    fn cannot_add_same_member_twice() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();

        let result = community.add_member(
            &owner_id,
            member_id,
            Role::Member,
            None,
        );

        assert_eq!(result, Err(CommunityError::AlreadyMember));
    }

    #[test]
    fn owner_can_activate_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();

        let result =
            community.activate_member(&owner_id, &member_id);

        assert!(result.is_ok());

        let member = community.member(&member_id).unwrap();
        assert_eq!(member.status(), MembershipStatus::Active);
    }

    #[test]
    fn owner_can_change_member_role() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();
        community
            .activate_member(&owner_id, &member_id)
            .unwrap();

        let result = community.change_member_role(
            &owner_id,
            &member_id,
            Role::Admin,
        );

        assert!(result.is_ok());

        let member = community.member(&member_id).unwrap();
        assert_eq!(member.role(), Role::Admin);
    }

    #[test]
    fn cannot_change_owner_role() {
        let mut community = community();
        let owner_id = community.owner_id.clone();

        let result = community.change_member_role(
            &owner_id,
            &owner_id,
            Role::Admin,
        );

        assert_eq!(result, Err(CommunityError::CannotRemoveOwner));
    }

    #[test]
    fn admin_can_suspend_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();
        let admin_id = AccountId::generate();

        community
            .add_member(&owner_id, admin_id.clone(), Role::Admin, None)
            .unwrap();
        community
            .activate_member(&owner_id, &admin_id)
            .unwrap();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();
        community
            .activate_member(&owner_id, &member_id)
            .unwrap();

        let result =
            community.suspend_member(&admin_id, &member_id);

        assert!(result.is_ok());

        let member = community.member(&member_id).unwrap();
        assert_eq!(member.status(), MembershipStatus::Suspended);
    }

    #[test]
    fn cannot_suspend_owner() {
        let mut community = community();
        let owner_id = community.owner_id.clone();

        let result =
            community.suspend_member(&owner_id, &owner_id);

        assert_eq!(result, Err(CommunityError::CannotRemoveOwner));
    }

    #[test]
    fn owner_can_ban_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();
        community
            .activate_member(&owner_id, &member_id)
            .unwrap();

        let result =
            community.ban_member(&owner_id, &member_id);

        assert!(result.is_ok());

        let member = community.member(&member_id).unwrap();
        assert_eq!(member.status(), MembershipStatus::Banned);
    }

    #[test]
    fn owner_can_remove_member() {
        let mut community = community();
        let owner_id = community.owner_id.clone();
        let member_id = AccountId::generate();

        community
            .add_member(&owner_id, member_id.clone(), Role::Member, None)
            .unwrap();

        let result =
            community.remove_member(&owner_id, &member_id);

        assert!(result.is_ok());
        assert!(!community.is_member(&member_id));
    }

    #[test]
    fn cannot_remove_owner() {
        let mut community = community();
        let owner_id = community.owner_id.clone();

        let result =
            community.remove_member(&owner_id, &owner_id);

        assert_eq!(result, Err(CommunityError::CannotRemoveOwner));
    }
}


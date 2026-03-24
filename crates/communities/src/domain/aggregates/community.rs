use crate::domain::{
    policies::membership_policy::MembershipPolicy,
    value_objects::{
        community_id::CommunityId, community_name::CommunityName, community_slug::CommunitySlug,
    },
};
use iam::domain::value_objects::AccountId;

#[derive(Debug, Clone)]
pub struct Community {
    id: CommunityId,
    _owner_id: AccountId,
    slug: CommunitySlug,
    name: CommunityName,
    public: bool,
    membership_policy: Option<MembershipPolicy>,
}

impl Community {
    pub fn create(
        id: CommunityId,
        _owner_id: AccountId,
        slug: CommunitySlug,
        name: CommunityName,
        public: bool,
    ) -> Self {
        Self {
            id,
            _owner_id,
            slug,
            name,
            public,
            membership_policy: None,
        }
    }

    pub fn id(&self) -> &CommunityId {
        &self.id
    }

    pub fn slug(&self) -> &CommunitySlug {
        &self.slug
    }

    pub fn name(&self) -> &CommunityName {
        &self.name
    }

    pub fn is_public(&self) -> bool {
        self.public
    }

    pub fn membership_policy(&self) -> &Option<MembershipPolicy> {
        &self.membership_policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Community {
        pub fn dummy_community() -> Community {
            Community::create(
                CommunityId::generate(),
                AccountId::generate(),
                CommunitySlug::new("rust-community".to_string()).unwrap(),
                CommunityName::new("Rust Community".to_string()).unwrap(),
                true,
            )
        }

        pub fn dummy_private_community() -> Community {
            Community::create(
                CommunityId::generate(),
                AccountId::generate(),
                CommunitySlug::new("rust-community".to_string()).unwrap(),
                CommunityName::new("Rust Community".to_string()).unwrap(),
                false,
            )
        }
    }
}

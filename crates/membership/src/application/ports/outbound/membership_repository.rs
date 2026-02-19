use crate::domain::entities::membership::Membership;

pub trait MembershipRepositoryPort: Send + Sync {
    fn save(&self, membership: &Membership) -> Result<(), String>;
}


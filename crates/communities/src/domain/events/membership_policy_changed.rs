use crate::domain::policies::membership_policy::MembershipPolicy;

#[derive(Debug, Clone)]
pub struct MembershipPolicyChanged {
    pub new_policy: MembershipPolicy,
}

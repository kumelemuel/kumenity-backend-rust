#[derive(Debug, Clone)]
pub enum MembershipPolicy {
    Open,
    ByInvitation,
    ByApplication,
    Closed,
}

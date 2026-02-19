#[derive(Debug, PartialEq, Eq)]
pub enum CommunityError {
    NotOwner,
    NotMember,
    AlreadyMember,
    CannotRemoveOwner,
    InsufficientPermissions,
    InvalidState,
}

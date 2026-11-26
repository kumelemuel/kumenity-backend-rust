// Codes of `InvalidCommunityId` and of the dead `CommunityError` (HEX-7.12),
// still fed to `ErrorLayout` until R19. The codes `CreateCommunity` answers
// live in `infrastructure/http/codes.rs` since R17 (ERR-4.4).
pub const COMMUNITIES_INVALID_COMMUNITY_ID: &str = "COMMUNITIES_INVALID_COMMUNITY_ID";
pub const COMMUNITIES_NOT_OWNER: &str = "COMMUNITIES_NOT_OWNER";
pub const COMMUNITIES_NOT_MEMBER: &str = "COMMUNITIES_NOT_MEMBER";
pub const COMMUNITIES_ALREADY_MEMBER: &str = "COMMUNITIES_ALREADY_MEMBER";
pub const COMMUNITIES_CANNOT_REMOVE_OWNER: &str = "COMMUNITIES_CANNOT_REMOVE_OWNER";
pub const COMMUNITIES_INSUFFICIENT_PERMISSIONS: &str = "COMMUNITIES_INSUFFICIENT_PERMISSIONS";
pub const COMMUNITIES_INVALID_STATE: &str = "COMMUNITIES_INVALID_STATE";

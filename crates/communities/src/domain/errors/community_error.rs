use super::error_codes::{
    COMMUNITIES_ALREADY_MEMBER, COMMUNITIES_CANNOT_REMOVE_OWNER,
    COMMUNITIES_INSUFFICIENT_PERMISSIONS, COMMUNITIES_INVALID_STATE, COMMUNITIES_NOT_MEMBER,
    COMMUNITIES_NOT_OWNER,
};
use shared::error::{ErrorCategory, LayerError};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CommunityError {
    NotOwner,
    NotMember,
    AlreadyMember,
    CannotRemoveOwner,
    InsufficientPermissions,
    InvalidState,
}

impl fmt::Display for CommunityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommunityError::NotOwner => write!(f, "Only the owner can perform this action"),
            CommunityError::NotMember => write!(f, "You must be a member to perform this action"),
            CommunityError::AlreadyMember => {
                write!(f, "You are already a member of this community")
            }
            CommunityError::CannotRemoveOwner => write!(f, "The community owner cannot be removed"),
            CommunityError::InsufficientPermissions => {
                write!(f, "You do not have permission for this action")
            }
            CommunityError::InvalidState => {
                write!(f, "This action is not allowed in the current state")
            }
        }
    }
}

impl std::error::Error for CommunityError {}

impl LayerError for CommunityError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Domain
    }

    fn code(&self) -> &'static str {
        match self {
            CommunityError::NotOwner => COMMUNITIES_NOT_OWNER,
            CommunityError::NotMember => COMMUNITIES_NOT_MEMBER,
            CommunityError::AlreadyMember => COMMUNITIES_ALREADY_MEMBER,
            CommunityError::CannotRemoveOwner => COMMUNITIES_CANNOT_REMOVE_OWNER,
            CommunityError::InsufficientPermissions => COMMUNITIES_INSUFFICIENT_PERMISSIONS,
            CommunityError::InvalidState => COMMUNITIES_INVALID_STATE,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            CommunityError::NotOwner => "Only the owner can perform this action.",
            CommunityError::NotMember => "You must be a member to perform this action.",
            CommunityError::AlreadyMember => "You are already a member of this community.",
            CommunityError::CannotRemoveOwner => "The community owner cannot be removed.",
            CommunityError::InsufficientPermissions => {
                "You do not have permission for this action."
            }
            CommunityError::InvalidState => "This action is not allowed in the current state.",
        }
    }
}

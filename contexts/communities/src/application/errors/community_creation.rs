use super::error_codes::COMMUNITIES_SLUG_ALREADY_EXISTS;
use shared::error::{ArchitectureLayer, ErrorLayout};
use std::fmt;

#[derive(Debug)]
pub enum CommunityCreationError {
    SlugAlreadyExists,
}

impl fmt::Display for CommunityCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommunityCreationError::SlugAlreadyExists => {
                write!(f, "This community slug is already in use")
            }
        }
    }
}

impl std::error::Error for CommunityCreationError {}

impl ErrorLayout for CommunityCreationError {
    fn architecture_layer(&self) -> ArchitectureLayer {
        ArchitectureLayer::Application
    }

    fn code(&self) -> &'static str {
        match self {
            CommunityCreationError::SlugAlreadyExists => COMMUNITIES_SLUG_ALREADY_EXISTS,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            CommunityCreationError::SlugAlreadyExists => "This community slug is already in use.",
        }
    }
}

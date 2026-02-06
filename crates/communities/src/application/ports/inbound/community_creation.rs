use shared::application::auth_context::AuthContext;
use crate::application::commands::create_community::CreateCommunity;
use crate::application::errors::application_error::ApplicationError;
use crate::application::results::community_created::CommunityCreated;

pub trait CommunityCreationPort {
    fn execute(&self, data: CreateCommunity, auth: AuthContext) -> Result<CommunityCreated, ApplicationError>;
}

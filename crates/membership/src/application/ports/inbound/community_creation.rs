use crate::application::{
    commands::create_community::CreateCommunity, errors::application_error::ApplicationError,
    results::community_created::CommunityCreated,
};
use shared::application::auth_context::AuthContext;

pub trait CommunityCreationPort {
    fn execute(
        &self,
        data: CreateCommunity,
        auth: AuthContext,
    ) -> Result<CommunityCreated, ApplicationError>;
}

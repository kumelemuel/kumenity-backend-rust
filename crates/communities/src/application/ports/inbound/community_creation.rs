use crate::application::{
    commands::create_community::CreateCommunity, results::community_created::CommunityCreated,
};
use shared::{application::auth_context::AuthContext, error::SystemError};

pub trait CommunityCreationPort {
    fn execute(
        &self,
        data: CreateCommunity,
        auth: AuthContext,
    ) -> Result<CommunityCreated, SystemError>;
}

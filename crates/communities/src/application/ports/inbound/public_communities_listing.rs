use crate::application::{
    commands::list_public_communities::ListPublicCommunities,
    results::public_communities_listed::PublicCommunitiesListed,
};
use shared::{application::auth_context::AuthContext, error::SystemError};

pub trait PublicCommunitiesListingPort {
    fn execute(
        &self,
        data: ListPublicCommunities,
        auth: AuthContext,
    ) -> Result<PublicCommunitiesListed, SystemError>;
}

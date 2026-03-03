use shared::application::auth_context::AuthContext;

use crate::application::commands::list_public_communities::ListPublicCommunities;
use crate::application::errors::application_error::ApplicationError;
use crate::application::results::public_communities_listed::PublicCommunitiesListed;

pub trait PublicCommunitiesListingPort {
    fn execute(
        &self,
        data: ListPublicCommunities,
        auth: AuthContext,
    ) -> Result<PublicCommunitiesListed, ApplicationError>;
}

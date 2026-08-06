use crate::application::queries::list_public_communities::{
    query::ListPublicCommunitiesQuery, response::ListPublicCommunitiesResponse,
};
use shared::{application::auth_context::AuthContext, error::SystemError};

pub trait ListPublicCommunitiesUseCase {
    fn execute(
        &self,
        data: ListPublicCommunitiesQuery,
        auth: AuthContext,
    ) -> Result<ListPublicCommunitiesResponse, SystemError>;
}

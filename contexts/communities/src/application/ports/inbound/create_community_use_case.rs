use crate::application::commands::create_community::{
    CreateCommunityCommand, CreateCommunityResponse,
};
use shared::{domain::model::verified_identity::VerifiedIdentity, error::SystemError};

pub trait CreateCommunityUseCase: Send + Sync {
    fn execute(
        &self,
        data: CreateCommunityCommand,
        auth: VerifiedIdentity,
    ) -> Result<CreateCommunityResponse, SystemError>;
}

use crate::application::commands::create_community::{
    CreateCommunityCommand, CreateCommunityError, CreateCommunityResponse,
};
use shared::Failure;

pub trait CreateCommunityUseCase: Send + Sync {
    fn execute(
        &self,
        command: CreateCommunityCommand,
    ) -> Result<CreateCommunityResponse, Failure<CreateCommunityError>>;
}

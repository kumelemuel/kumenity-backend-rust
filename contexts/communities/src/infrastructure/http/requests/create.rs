use crate::application::commands::create_community::command::CreateCommunityCommand;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub slug: String,
    pub name: String,
    pub is_public: bool,
}

impl From<CreateRequest> for CreateCommunityCommand {
    fn from(req: CreateRequest) -> Self {
        CreateCommunityCommand {
            slug: req.slug,
            name: req.name,
            is_public: req.is_public,
        }
    }
}

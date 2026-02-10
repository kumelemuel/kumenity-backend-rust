
use serde::Deserialize;
use communities::application::commands::create_community::CreateCommunity;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub slug: String,
    pub name: String,
    pub is_public: bool,
}

impl From<CreateRequest> for CreateCommunity {
    fn from(req: CreateRequest) -> Self {
        CreateCommunity {
            slug: req.slug,
            name: req.name,
            is_public: req.is_public,
        }
    }
}

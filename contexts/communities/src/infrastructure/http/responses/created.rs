use crate::application::commands::create_community::response::CreateCommunityResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl From<CreateCommunityResponse> for CreatedResponse {
    fn from(dto: CreateCommunityResponse) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
        }
    }
}

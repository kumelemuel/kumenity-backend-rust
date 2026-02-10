use serde::Serialize;
use communities::application::results::community_created::CommunityCreated;

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl From<CommunityCreated> for CreatedResponse {
    fn from(dto: CommunityCreated) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
        }
    }
}

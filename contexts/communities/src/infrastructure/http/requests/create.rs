use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub slug: String,
    pub name: String,
    pub is_public: bool,
}

pub struct PublicCommunitiesListed {
    pub communities: Vec<CommunityResult>,
}

pub struct CommunityResult {
    pub name: String,
    pub slug: String,
}

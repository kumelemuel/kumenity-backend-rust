pub struct CreateCommunityCommand {
    /// Subject of the verified identity of the caller. It travels inside the
    /// command (HEX-4): `execute` receives nothing else.
    pub owner_id: String,
    pub slug: String,
    pub name: String,
    pub is_public: bool,
}

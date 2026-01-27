pub trait TokenGeneratorPort: Send + Sync {
    fn generate(&self, user_id: &str) -> Result<String, String>;
}

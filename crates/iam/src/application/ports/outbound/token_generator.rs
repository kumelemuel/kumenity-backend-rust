pub trait TokenGeneratorPort: Send + Sync {
    fn generate(&self, user_id: &str) -> Result<String, String>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::token_generator::TokenGeneratorPort;

    pub struct FakeTokenGenerator;

    impl TokenGeneratorPort for FakeTokenGenerator {
        fn generate(&self, _user_id: &str) -> Result<String, String> {
            Ok(String::from("valid_token"))
        }
    }
}
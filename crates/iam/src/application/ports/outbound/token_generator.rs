use crate::application::errors::token_generator::TokenGeneratorError;

pub trait TokenGeneratorPort: Send + Sync {
    fn generate(&self, account_id: &str) -> Result<String, TokenGeneratorError>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::{
        errors::token_generator::TokenGeneratorError,
        ports::outbound::token_generator::TokenGeneratorPort,
    };

    pub struct FakeTokenGenerator;

    impl TokenGeneratorPort for FakeTokenGenerator {
        fn generate(&self, _account_id: &str) -> Result<String, TokenGeneratorError> {
            Ok(String::from("valid_token"))
        }
    }
}

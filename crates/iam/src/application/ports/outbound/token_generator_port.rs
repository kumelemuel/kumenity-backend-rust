use crate::domain::value_objects::UserId;

pub trait TokenGeneratorPort: Send + Sync {
    fn generate(&self, user_id: &UserId) -> String;
}

use crate::users::domain::value_objects::{UserId, Email};

#[derive(Debug, Clone)]
pub struct UserCreated {
    pub id: UserId,
    pub email: Email,
}
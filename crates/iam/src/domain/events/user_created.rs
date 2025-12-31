use crate::domain::value_objects::{Email, UserId};

#[derive(Debug, Clone)]
pub struct UserCreated {
    pub id: UserId,
    pub email: Email,
}

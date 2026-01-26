use crate::domain::value_objects::{Email, AccountId};

#[derive(Debug, Clone)]
pub struct UserCreated {
    pub id: AccountId,
    pub email: Email,
}

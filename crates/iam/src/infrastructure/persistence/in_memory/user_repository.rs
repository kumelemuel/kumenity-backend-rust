use crate::application::ports::outbound::user_repository_port::UserRepositoryPort;
use crate::domain::aggregates::User;
use crate::domain::value_objects::UserId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<UserId, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl UserRepositoryPort for InMemoryUserRepository {
    fn find_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.lock().expect("mutex poisoned");
        users
            .values()
            .find(|user| user.username().as_str() == username)
            .cloned()
    }

    fn find_by_email(&self, email: &str) -> Option<User> {
        let users = self.users.lock().expect("mutex poisoned");
        users
            .values()
            .find(|user| user.email().as_str() == email)
            .cloned()
    }

    fn save(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.lock().expect("mutex poisoned");

        users.insert(user.id().clone(), user.clone());

        Ok(())
    }
}

use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::domain::aggregates::Account;
use crate::domain::value_objects::AccountId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<AccountId, Account>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AccountRepositoryPort for InMemoryUserRepository {
    fn find_by_username(&self, username: &str) -> Option<Account> {
        let users = self.users.lock().expect("mutex poisoned");
        users
            .values()
            .find(|user| user.username().as_str() == username)
            .cloned()
    }

    fn find_by_email(&self, email: &str) -> Option<Account> {
        let users = self.users.lock().expect("mutex poisoned");
        users
            .values()
            .find(|user| user.email().as_str() == email)
            .cloned()
    }

    fn save(&self, user: &Account) -> Result<(), String> {
        let mut users = self.users.lock().expect("mutex poisoned");

        users.insert(user.id().clone(), user.clone());

        Ok(())
    }
}

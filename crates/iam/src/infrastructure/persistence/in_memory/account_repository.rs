use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
use crate::domain::aggregates::Account;
use crate::domain::value_objects::AccountId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryAccountRepository {
    accounts: Arc<Mutex<HashMap<AccountId, Account>>>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AccountRepositoryPort for InMemoryAccountRepository {
    fn find_by_username(&self, username: &str) -> Option<Account> {
        let accounts = self.accounts.lock().expect("mutex poisoned");
        accounts
            .values()
            .find(|account| account.username().as_str() == username)
            .cloned()
    }

    fn find_by_email(&self, email: &str) -> Option<Account> {
        let accounts = self.accounts.lock().expect("mutex poisoned");
        accounts
            .values()
            .find(|account| account.email().as_str() == email)
            .cloned()
    }

    fn save(&self, account: &Account) -> Result<(), String> {
        let mut accounts = self.accounts.lock().expect("mutex poisoned");

        accounts.insert(account.id().clone(), account.clone());

        Ok(())
    }
}

use crate::{
    application::{
        errors::account_repository::AccountRepositoryError,
        ports::outbound::account_repository::AccountRepositoryPort,
    },
    domain::{aggregates::Account, value_objects::AccountId},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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

    fn save(&self, account: &Account) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.accounts.lock().expect("mutex poisoned");

        accounts.insert(account.id().clone(), account.clone());

        Ok(())
    }
}

use crate::{
    application::ports::outbound::account_repository::AccountRepository,
    domain::model::account::{account::Account, account_id::AccountId},
};
use shared::{Fault, domain::value_object::ValueObject};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct InMemoryAccountRepository {
    accounts: Arc<Mutex<HashMap<AccountId, Account>>>,
}

impl Default for InMemoryAccountRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AccountRepository for InMemoryAccountRepository {
    fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault> {
        let accounts = self.accounts.lock().expect("mutex poisoned");
        Ok(accounts
            .values()
            .find(|account| account.username().value() == username)
            .cloned())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault> {
        let accounts = self.accounts.lock().expect("mutex poisoned");
        Ok(accounts
            .values()
            .find(|account| account.email().value() == email)
            .cloned())
    }

    fn save(&self, account: &Account) -> Result<(), Fault> {
        let mut accounts = self.accounts.lock().expect("mutex poisoned");

        accounts.insert(account.id().clone(), account.clone());

        Ok(())
    }
}

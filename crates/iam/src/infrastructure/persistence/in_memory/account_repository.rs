use crate::{
    application::{
        errors::account_repository::AccountRepositoryError,
        ports::outbound::account_repository::AccountRepositoryPort,
    },
    domain::{aggregates::Account, value_objects::AccountId},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
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
    /// Each operation leaves the map consistent (a single read or `insert`),
    /// so a panic in another thread cannot leave it half-written: recovering
    /// the guard is safe and keeps one panic from poisoning every request.
    fn accounts(&self) -> MutexGuard<'_, HashMap<AccountId, Account>> {
        self.accounts.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl AccountRepositoryPort for InMemoryAccountRepository {
    fn find_by_username(&self, username: &str) -> Option<Account> {
        let accounts = self.accounts();
        accounts
            .values()
            .find(|account| account.username().as_str() == username)
            .cloned()
    }

    fn find_by_email(&self, email: &str) -> Option<Account> {
        let accounts = self.accounts();
        accounts
            .values()
            .find(|account| account.email().as_str() == email)
            .cloned()
    }

    fn save(&self, account: &Account) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.accounts();

        accounts.insert(account.id().clone(), account.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_serving_after_a_panic_while_the_lock_was_held() {
        let repository = InMemoryAccountRepository::new();
        let account = Account::dummy_account();
        repository.save(&account).unwrap();

        let accounts = Arc::clone(&repository.accounts);
        let _ = std::thread::spawn(move || {
            let _guard = accounts.lock().unwrap();
            panic!("panic while holding the lock");
        })
        .join();

        assert!(repository.find_by_username("dummy").is_some());
        assert!(repository.find_by_email("dummy@example.com").is_some());
        assert!(repository.save(&account).is_ok());
    }
}

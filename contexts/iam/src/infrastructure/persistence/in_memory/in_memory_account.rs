use crate::{
    application::ports::outbound::account_repository::AccountRepository,
    domain::model::account::{account::Account, account_id::AccountId},
};
use shared::{Fault, domain::value_object::ValueObject};
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

/// Adaptador en memoria de `AccountRepository`.
///
/// Casi igual a `FakeAccountAdapter` mientras no haya persistencia real. No se
/// fusionan: este es un adaptador de infraestructura (de producción); aquel,
/// un doble de application (TDD-2).
#[derive(Default)]
pub struct InMemoryAccountAdapter {
    accounts: Mutex<HashMap<AccountId, Account>>,
}

impl InMemoryAccountAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    fn store(&self) -> MutexGuard<'_, HashMap<AccountId, Account>> {
        self.accounts
            .lock()
            .expect("invariante: mutex sin envenenar")
    }
}

impl AccountRepository for InMemoryAccountAdapter {
    fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault> {
        Ok(self
            .store()
            .values()
            .find(|account| account.username().value() == username)
            .cloned())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault> {
        Ok(self
            .store()
            .values()
            .find(|account| account.email().value() == email)
            .cloned())
    }

    fn save(&self, account: &Account) -> Result<(), Fault> {
        self.store().insert(account.id().clone(), account.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::testing::block_on;

    crate::account_repository_contract! {
        new_adapter: InMemoryAccountAdapter::new(),
        run: block_on,
    }
}

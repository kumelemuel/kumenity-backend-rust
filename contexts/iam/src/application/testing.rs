//! Dobles de los puertos salientes y baterías de contrato (TDD-2): código de
//! primera clase.
//!
//! Los dobles se nombran como adaptadores (HEX-5) con un prefijo de la lista
//! cerrada de TDD-2: `Fake` si tienen estado, `Fixed` o `Stub` si responden
//! fijo, `Spy` si registran llamadas. Disponibles para los tests de este crate
//! y, con la feature `testing`, para los de otros crates.

use crate::{
    application::ports::outbound::account_repository::AccountRepository,
    domain::model::account::account::Account,
};
use shared::{Fault, domain::value_object::ValueObject};
use std::sync::{Mutex, MutexGuard};

pub mod contracts;

/// Operación de `AccountRepository`, para hacer fallar solo esa en un
/// `FakeAccountAdapter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountOperation {
    FindByUsername,
    FindByEmail,
    Save,
}

/// Doble de `AccountRepository`: guarda en memoria, o falla.
///
/// `failing()` hace fallar todas las operaciones, como el template. Para que
/// un handler llegue a una llamada posterior a la primera y falle esa,
/// `failing_on(op)`: las demás operaciones siguen el contrato (buscan por el
/// valor pedido, `save` guarda).
///
/// Casi igual a `InMemoryAccountAdapter` mientras no haya persistencia real.
/// No se fusionan: este es un doble de application; aquel, un adaptador de
/// infraestructura (TDD-2).
pub struct FakeAccountAdapter {
    accounts: Mutex<Vec<Account>>,
    failing: Vec<AccountOperation>,
}

impl FakeAccountAdapter {
    pub fn empty() -> Self {
        Self::containing([])
    }

    /// Con estas cuentas ya guardadas.
    pub fn containing(accounts: impl IntoIterator<Item = Account>) -> Self {
        Self {
            accounts: Mutex::new(accounts.into_iter().collect()),
            failing: Vec::new(),
        }
    }

    /// Simula una falla técnica no declarada (árbol B) en todas las
    /// operaciones.
    pub fn failing() -> Self {
        Self {
            failing: vec![
                AccountOperation::FindByUsername,
                AccountOperation::FindByEmail,
                AccountOperation::Save,
            ],
            ..Self::empty()
        }
    }

    /// Además, `operation` responde un `Fault` (árbol B).
    pub fn failing_on(mut self, operation: AccountOperation) -> Self {
        self.failing.push(operation);
        self
    }

    fn store(&self, operation: AccountOperation) -> Result<MutexGuard<'_, Vec<Account>>, Fault> {
        if self.failing.contains(&operation) {
            return Err(Fault::new("fake repository outage"));
        }
        Ok(self
            .accounts
            .lock()
            .expect("invariante: mutex sin envenenar"))
    }
}

impl AccountRepository for FakeAccountAdapter {
    fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault> {
        let accounts = self.store(AccountOperation::FindByUsername)?;
        Ok(accounts
            .iter()
            .find(|account| account.username().value() == username)
            .cloned())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault> {
        let accounts = self.store(AccountOperation::FindByEmail)?;
        Ok(accounts
            .iter()
            .find(|account| account.email().value() == email)
            .cloned())
    }

    fn save(&self, account: &Account) -> Result<(), Fault> {
        let mut accounts = self.store(AccountOperation::Save)?;
        // Mismo id: reemplaza, como cualquier repositorio (contrato).
        match accounts.iter_mut().find(|a| a.id() == account.id()) {
            Some(existing) => *existing = account.clone(),
            None => accounts.push(account.clone()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::account_repository_contract! {
        new_adapter: FakeAccountAdapter::empty(),
    }
}

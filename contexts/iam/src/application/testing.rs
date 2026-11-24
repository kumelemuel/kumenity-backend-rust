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

/// Doble de `AccountRepository`: guarda en memoria, o falla siempre.
///
/// Casi igual a `InMemoryAccountAdapter` mientras no haya persistencia real.
/// No se fusionan: este es un doble de application; aquel, un adaptador de
/// infraestructura (TDD-2).
pub struct FakeAccountAdapter {
    accounts: Mutex<Vec<Account>>,
    failing: bool,
}

impl FakeAccountAdapter {
    pub fn empty() -> Self {
        Self {
            accounts: Mutex::new(Vec::new()),
            failing: false,
        }
    }

    /// Con estas cuentas ya guardadas.
    pub fn containing(accounts: impl IntoIterator<Item = Account>) -> Self {
        Self {
            accounts: Mutex::new(accounts.into_iter().collect()),
            failing: false,
        }
    }

    /// Simula una falla técnica no declarada (árbol B) en todas las
    /// operaciones. Para que falle una llamada posterior a la primera, usar
    /// `StubAccountAdapter`.
    pub fn failing() -> Self {
        Self {
            accounts: Mutex::new(Vec::new()),
            failing: true,
        }
    }

    fn store(&self) -> Result<MutexGuard<'_, Vec<Account>>, Fault> {
        if self.failing {
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
        let accounts = self.store()?;
        Ok(accounts
            .iter()
            .find(|account| account.username().value() == username)
            .cloned())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault> {
        let accounts = self.store()?;
        Ok(accounts
            .iter()
            .find(|account| account.email().value() == email)
            .cloned())
    }

    fn save(&self, account: &Account) -> Result<(), Fault> {
        let mut accounts = self.store()?;
        // Mismo id: reemplaza, como cualquier repositorio (contrato).
        match accounts.iter_mut().find(|a| a.id() == account.id()) {
            Some(existing) => *existing = account.clone(),
            None => accounts.push(account.clone()),
        }
        Ok(())
    }
}

/// Operación de `AccountRepository` que falla en un `StubAccountAdapter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountOperation {
    FindByUsername,
    FindByEmail,
    Save,
}

/// Doble de `AccountRepository` con respuesta fija: solo la operación
/// `failing_on` responde un `Fault`. Las búsquedas que no fallan devuelven la
/// cuenta de `finding` (o `None`), sin importar el valor buscado, y `save` no
/// guarda nada.
///
/// Sirve para que un handler llegue a una llamada posterior a la primera y
/// esa sea la que falla. No pasa la batería de contrato: no tiene estado, a
/// propósito (TDD-2, `Stub`).
pub struct StubAccountAdapter {
    found: Option<Account>,
    failing_on: AccountOperation,
}

impl StubAccountAdapter {
    pub fn failing_on(operation: AccountOperation) -> Self {
        Self {
            found: None,
            failing_on: operation,
        }
    }

    /// Las búsquedas que no fallan devuelven esta cuenta.
    pub fn finding(self, account: Account) -> Self {
        Self {
            found: Some(account),
            ..self
        }
    }

    fn answer<T>(&self, operation: AccountOperation, ok: T) -> Result<T, Fault> {
        if self.failing_on == operation {
            return Err(Fault::new("stub repository outage"));
        }
        Ok(ok)
    }
}

impl AccountRepository for StubAccountAdapter {
    fn find_by_username(&self, _username: &str) -> Result<Option<Account>, Fault> {
        self.answer(AccountOperation::FindByUsername, self.found.clone())
    }

    fn find_by_email(&self, _email: &str) -> Result<Option<Account>, Fault> {
        self.answer(AccountOperation::FindByEmail, self.found.clone())
    }

    fn save(&self, _account: &Account) -> Result<(), Fault> {
        self.answer(AccountOperation::Save, ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::account_repository_contract! {
        new_adapter: FakeAccountAdapter::empty(),
    }
}

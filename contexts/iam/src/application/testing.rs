//! Dobles de los puertos salientes y baterías de contrato (TDD-2): código de
//! primera clase.
//!
//! Los dobles se nombran como adaptadores (HEX-5) con un prefijo de la lista
//! cerrada de TDD-2: `Fake` si tienen estado, `Fixed` o `Stub` si responden
//! fijo, `Spy` si registran llamadas. Disponibles para los tests de este crate
//! y, con la feature `testing`, para los de otros crates.

use crate::{
    application::ports::outbound::{
        account_repository::AccountRepository, password_hashing_service::PasswordHashingService,
    },
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

/// Operación de `PasswordHashingService`, para hacer fallar solo esa en un
/// `FixedPasswordHashingAdapter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordHashingOperation {
    Hash,
    Verify,
}

/// Marca de los hashes del doble. Larga para que el hash sea un
/// `HashedPassword` válido (contrato).
const FIXED_HASH_PREFIX: &str = "$fixed-password-hashing-adapter$v=1$not-a-real-hash$";

/// Doble de `PasswordHashingService`, determinista y coherente (TDD-2.4): el
/// hash se deriva de la contraseña y `verify` solo acepta la contraseña con
/// la que se generó. Un hash que no produjo es un `Fault`, como un hash
/// corrupto en el adaptador real.
///
/// No es criptográfico: el hash contiene la contraseña. Solo para tests.
///
/// Fallas como en `FakeAccountAdapter`: `failing()` hace fallar todo;
/// `failing_on(op)`, solo esa operación.
#[derive(Default)]
pub struct FixedPasswordHashingAdapter {
    failing: Vec<PasswordHashingOperation>,
}

impl FixedPasswordHashingAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Simula una falla técnica no declarada (árbol B) en todas las
    /// operaciones.
    pub fn failing() -> Self {
        Self {
            failing: vec![
                PasswordHashingOperation::Hash,
                PasswordHashingOperation::Verify,
            ],
        }
    }

    /// Además, `operation` responde un `Fault` (árbol B).
    pub fn failing_on(mut self, operation: PasswordHashingOperation) -> Self {
        self.failing.push(operation);
        self
    }

    fn check(&self, operation: PasswordHashingOperation) -> Result<(), Fault> {
        if self.failing.contains(&operation) {
            return Err(Fault::new("fixed password hashing outage"));
        }
        Ok(())
    }
}

impl PasswordHashingService for FixedPasswordHashingAdapter {
    fn hash(&self, password: &str) -> Result<String, Fault> {
        self.check(PasswordHashingOperation::Hash)?;
        Ok(format!("{FIXED_HASH_PREFIX}{password}"))
    }

    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
        self.check(PasswordHashingOperation::Verify)?;
        let hashed = hashed_password
            .strip_prefix(FIXED_HASH_PREFIX)
            .ok_or_else(|| Fault::new("fixed password hashing: unreadable hash"))?;
        Ok(hashed == password)
    }
}

/// Doble de `PasswordHashingService` que registra cada `verify` y responde
/// como `FixedPasswordHashingAdapter`, así que pasa la misma batería.
#[derive(Default)]
pub struct SpyPasswordHashingAdapter {
    hasher: FixedPasswordHashingAdapter,
    verify_calls: Mutex<Vec<(String, String)>>,
}

impl SpyPasswordHashingAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// `(contraseña, hash)` de cada `verify`, en orden.
    pub fn verify_calls(&self) -> Vec<(String, String)> {
        self.verify_calls
            .lock()
            .expect("invariante: mutex sin envenenar")
            .clone()
    }
}

impl PasswordHashingService for SpyPasswordHashingAdapter {
    fn hash(&self, password: &str) -> Result<String, Fault> {
        self.hasher.hash(password)
    }

    fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Fault> {
        self.verify_calls
            .lock()
            .expect("invariante: mutex sin envenenar")
            .push((password.to_string(), hashed_password.to_string()));
        self.hasher.verify(password, hashed_password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fake_account {
        use super::*;

        crate::account_repository_contract! {
            new_adapter: FakeAccountAdapter::empty(),
        }
    }

    mod fixed_password_hashing {
        use super::*;

        crate::password_hashing_service_contract! {
            new_service: FixedPasswordHashingAdapter::new(),
        }
    }

    mod spy_password_hashing {
        use super::*;

        crate::password_hashing_service_contract! {
            new_service: SpyPasswordHashingAdapter::new(),
        }
    }
}

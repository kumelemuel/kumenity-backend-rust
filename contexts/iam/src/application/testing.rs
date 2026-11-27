//! Dobles de los puertos salientes y baterías de contrato (TDD-2): código de
//! primera clase.
//!
//! Los dobles se nombran como adaptadores (HEX-5) con un prefijo de la lista
//! cerrada de TDD-2: `Fake` si tienen estado, `Fixed` o `Stub` si responden
//! fijo, `Spy` si registran llamadas. Disponibles para los tests de este crate
//! y, con la feature `testing`, para los de otros crates.

use crate::{
    application::ports::outbound::{
        access_token_service::AccessTokenService, account_repository::AccountRepository,
        dtos::access_token_payloads::AccessTokenVerdict,
        password_hashing_service::PasswordHashingService,
        verification_code_service::VerificationCodeService,
    },
    domain::model::account::{
        Account, code_validation::CodeValidation, hashed_password::HashedPassword,
    },
};
use shared::{Fault, IdGenerator, domain::value_object::ValueObject};
use std::{
    collections::HashMap,
    future::Future,
    pin::pin,
    sync::{
        Mutex, MutexGuard,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll, Waker},
};

pub mod contracts;

/// Ejecuta un future sin runtime externo.
///
/// Pensado para fakes que resuelven de inmediato; no sirve para IO real.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::thread::yield_now();
    }
}

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
    fn hash(&self, password: &str) -> Result<HashedPassword, Fault> {
        self.check(PasswordHashingOperation::Hash)?;
        Ok(
            HashedPassword::new(format!("{FIXED_HASH_PREFIX}{password}"))
                .expect("invariante: el prefijo ya es más largo que lo que pide HashedPassword"),
        )
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
    fn hash(&self, password: &str) -> Result<HashedPassword, Fault> {
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

/// Operación de `AccessTokenService`, para hacer fallar solo esa en un
/// `FakeAccessTokenAdapter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessTokenOperation {
    Issue,
    Verify,
}

/// Un token emitido por `FakeAccessTokenAdapter`.
struct IssuedToken {
    account_id: String,
    expired: bool,
}

/// Doble de `AccessTokenService`: solo valen los tokens vigentes que emitió
/// esta instancia. Cada token es único entre todas las instancias, así que
/// otra instancia hace de emisor con otra clave: no acepta los tokens de esta,
/// ni los confunde con los suyos.
///
/// Fallas como en `FakeAccountAdapter`: `failing()` hace fallar todo;
/// `failing_on(op)`, solo esa operación.
#[derive(Default)]
pub struct FakeAccessTokenAdapter {
    issued: Mutex<HashMap<String, IssuedToken>>,
    failing: Vec<AccessTokenOperation>,
}

impl FakeAccessTokenAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Simula una falla técnica no declarada (árbol B) en todas las
    /// operaciones.
    pub fn failing() -> Self {
        Self {
            failing: vec![AccessTokenOperation::Issue, AccessTokenOperation::Verify],
            ..Self::default()
        }
    }

    /// Además, `operation` responde un `Fault` (árbol B).
    pub fn failing_on(mut self, operation: AccessTokenOperation) -> Self {
        self.failing.push(operation);
        self
    }

    /// Emite un token ya vencido para `account_id`, con la clave de esta
    /// instancia.
    pub fn issue_expired(&self, account_id: &str) -> String {
        self.record(account_id, true)
    }

    fn record(&self, account_id: &str, expired: bool) -> String {
        static NEXT_TOKEN: AtomicU64 = AtomicU64::new(0);
        let token = format!(
            "fake-access-token.{}",
            NEXT_TOKEN.fetch_add(1, Ordering::Relaxed)
        );
        let mut issued = self.issued.lock().expect("invariante: mutex sin envenenar");
        issued.insert(
            token.clone(),
            IssuedToken {
                account_id: account_id.to_string(),
                expired,
            },
        );
        token
    }

    fn check(&self, operation: AccessTokenOperation) -> Result<(), Fault> {
        if self.failing.contains(&operation) {
            return Err(Fault::new("fake access token outage"));
        }
        Ok(())
    }
}

impl AccessTokenService for FakeAccessTokenAdapter {
    fn issue(&self, account_id: &str) -> Result<String, Fault> {
        self.check(AccessTokenOperation::Issue)?;
        Ok(self.record(account_id, false))
    }

    fn verify(&self, token: &str) -> Result<AccessTokenVerdict, Fault> {
        self.check(AccessTokenOperation::Verify)?;
        let issued = self.issued.lock().expect("invariante: mutex sin envenenar");
        Ok(match issued.get(token) {
            Some(IssuedToken {
                account_id,
                expired: false,
            }) => AccessTokenVerdict::Valid {
                account_id: account_id.clone(),
            },
            _ => AccessTokenVerdict::Invalid,
        })
    }
}

/// Doble de `VerificationCodeService`: responde siempre el mismo código, o
/// falla. Una sola operación, así que no tiene `failing_on`.
pub struct FixedVerificationCodeAdapter {
    code: CodeValidation,
    failing: bool,
}

impl FixedVerificationCodeAdapter {
    pub fn new(code: u32) -> Self {
        Self {
            code: CodeValidation::new(code).expect("invariante: código de prueba de seis dígitos"),
            failing: false,
        }
    }

    /// Simula una falla técnica no declarada (árbol B).
    pub fn failing() -> Self {
        Self {
            failing: true,
            ..Self::new(123_123)
        }
    }
}

impl VerificationCodeService for FixedVerificationCodeAdapter {
    fn generate(&self) -> Result<CodeValidation, Fault> {
        if self.failing {
            return Err(Fault::new("fixed verification code outage"));
        }
        Ok(self.code)
    }
}

/// Answers always the same id (TDD-2: `Fixed`, fixed answer).
pub struct FixedIdGeneratorAdapter(pub &'static str);

impl IdGenerator for FixedIdGeneratorAdapter {
    fn next_id(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fake_account {
        use super::*;

        crate::account_repository_contract! {
            new_adapter: FakeAccountAdapter::empty(),
            run: block_on,
        }
    }

    mod fake_access_token {
        use super::*;

        crate::access_token_service_contract! {
            new_service: FakeAccessTokenAdapter::new(),
            expired_token: |service: &FakeAccessTokenAdapter, account_id: &str| {
                service.issue_expired(account_id)
            },
            another_issuer: FakeAccessTokenAdapter::new(),
            run: block_on,
        }
    }

    mod fixed_password_hashing {
        use super::*;

        crate::password_hashing_service_contract! {
            new_service: FixedPasswordHashingAdapter::new(),
            run: block_on,
        }
    }

    mod fixed_verification_code {
        use super::*;

        crate::verification_code_service_contract! {
            new_service: FixedVerificationCodeAdapter::new(123_123),
            run: block_on,
        }
    }

    mod spy_password_hashing {
        use super::*;

        crate::password_hashing_service_contract! {
            new_service: SpyPasswordHashingAdapter::new(),
            run: block_on,
        }
    }
}

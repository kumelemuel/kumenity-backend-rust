use crate::domain::model::account::Account;
use shared::Fault;

/// Persistencia de cuentas. No declara errores (ERR-3): cualquier falla técnica
/// es `Fault` (árbol B). "No existe" no es un error: `find_*` devuelve `None`.
pub trait AccountRepository: Send + Sync {
    fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault>;

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault>;

    fn save(&self, account: &Account) -> Result<(), Fault>;
}

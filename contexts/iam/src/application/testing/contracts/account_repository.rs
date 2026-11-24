//! Contrato de `AccountRepository`: el comportamiento del que dependen los
//! handlers. Lo pasan el doble con estado y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): cualquier falla es `Fault`, así que no
//! hay casos por variante ni escenarios de la fábrica para provocarlas.

use crate::{
    application::ports::outbound::account_repository::AccountRepository,
    domain::model::account::{
        account::Account, account_id::AccountId, account_status::AccountStatus,
        code_validation::CodeValidation, email::Email, hashed_password::HashedPassword,
        username::Username,
    },
};
use shared::domain::value_object::ValueObject;

const HASH: &str =
    "$2b$12$12345678901234567890123456789012345673434534534534346346346346346890123456789012";

fn account_with_id(id: AccountId, username: &str, email: &str) -> Account {
    Account::new(
        id,
        Username::new(username.into()).expect("invariante: username de prueba válido"),
        Email::new(email.into()).expect("invariante: email de prueba válido"),
        HashedPassword::new(HASH.into()).expect("invariante: hash de prueba válido"),
        AccountStatus::Registered {
            code_validation: CodeValidation::new(123_123)
                .expect("invariante: código de prueba válido"),
        },
    )
}

fn account(username: &str, email: &str) -> Account {
    account_with_id(AccountId::generate(), username, email)
}

type Snapshot = (AccountId, Username, Email, HashedPassword, AccountStatus);

/// `Account` no implementa `PartialEq`: se compara por lo que expone.
fn snapshot(account: &Account) -> Snapshot {
    (
        account.id().clone(),
        account.username().clone(),
        account.email().clone(),
        account.password().clone(),
        *account.status(),
    )
}

fn save(adapter: &impl AccountRepository, account: &Account) {
    adapter
        .save(account)
        .expect("contrato: save no falla con el adaptador sano");
}

fn by_email(adapter: &impl AccountRepository, value: &str) -> Option<Snapshot> {
    adapter
        .find_by_email(value)
        .expect("contrato: find_by_email no falla con el adaptador sano")
        .as_ref()
        .map(snapshot)
}

fn by_username(adapter: &impl AccountRepository, value: &str) -> Option<Snapshot> {
    adapter
        .find_by_username(value)
        .expect("contrato: find_by_username no falla con el adaptador sano")
        .as_ref()
        .map(snapshot)
}

/// Lo que se guarda se encuentra por su email, igual a como se guardó.
pub fn finds_by_email_what_was_saved<R: AccountRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    let saved = account("ana", "ana@example.com");

    save(&adapter, &saved);

    assert_eq!(
        by_email(&adapter, "ana@example.com"),
        Some(snapshot(&saved))
    );
}

/// Lo que se guarda se encuentra por su username, igual a como se guardó.
pub fn finds_by_username_what_was_saved<R: AccountRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    let saved = account("ana", "ana@example.com");

    save(&adapter, &saved);

    assert_eq!(by_username(&adapter, "ana"), Some(snapshot(&saved)));
}

/// "No existe" no es un error: `find_by_email` devuelve `None` (ERR-1).
pub fn unknown_email_is_none_not_an_error<R: AccountRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();

    assert_eq!(by_email(&adapter, "ana@example.com"), None);
}

/// "No existe" no es un error: `find_by_username` devuelve `None` (ERR-1).
pub fn unknown_username_is_none_not_an_error<R: AccountRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();

    assert_eq!(by_username(&adapter, "ana"), None);
}

/// El email se compara entero: ni prefijo ni extensión de uno guardado.
pub fn find_by_email_matches_the_whole_email<R: AccountRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    save(&adapter, &account("ana", "ana@example.com"));

    assert_eq!(by_email(&adapter, "ana@example"), None);
    assert_eq!(by_email(&adapter, "ana@example.com.ar"), None);
}

/// El username se compara entero: ni prefijo ni extensión de uno guardado.
pub fn find_by_username_matches_the_whole_username<R: AccountRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    save(&adapter, &account("anabel", "anabel@example.com"));

    assert_eq!(by_username(&adapter, "ana"), None);
    assert_eq!(by_username(&adapter, "anabella"), None);
}

/// Con varias cuentas, cada email y cada username encuentran la suya.
pub fn finds_each_account_by_its_own_email_and_username<R: AccountRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    let ana = account("ana", "ana@example.com");
    let bea = account("bea", "bea@example.com");
    save(&adapter, &ana);
    save(&adapter, &bea);

    assert_eq!(by_email(&adapter, "ana@example.com"), Some(snapshot(&ana)));
    assert_eq!(by_email(&adapter, "bea@example.com"), Some(snapshot(&bea)));
    assert_eq!(by_username(&adapter, "ana"), Some(snapshot(&ana)));
    assert_eq!(by_username(&adapter, "bea"), Some(snapshot(&bea)));
}

/// Guardar un agregado con un id existente lo reemplaza: no lo duplica ni deja
/// visible el estado anterior.
pub fn save_replaces_the_account_with_the_same_id<R: AccountRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    let id = AccountId::generate();
    save(
        &adapter,
        &account_with_id(id.clone(), "first", "first@example.com"),
    );
    let second = account_with_id(id, "second", "second@example.com");
    save(&adapter, &second);

    assert_eq!(by_email(&adapter, "first@example.com"), None);
    assert_eq!(by_username(&adapter, "first"), None);
    assert_eq!(
        by_email(&adapter, "second@example.com"),
        Some(snapshot(&second))
    );
}

/// Genera un `#[test]` por caso del contrato de `AccountRepository`.
///
/// `new_adapter`: expresión que construye un adaptador nuevo y vacío. Se
/// evalúa en un módulo hijo del que invoca la macro (`use super::*`).
///
/// Un caso nuevo se agrega aquí y como función arriba.
#[macro_export]
macro_rules! account_repository_contract {
    (new_adapter: $new_adapter:expr $(,)?) => {
        mod account_repository_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::account_repository as contract;

            #[test]
            fn finds_by_email_what_was_saved() {
                contract::finds_by_email_what_was_saved(|| $new_adapter);
            }

            #[test]
            fn finds_by_username_what_was_saved() {
                contract::finds_by_username_what_was_saved(|| $new_adapter);
            }

            #[test]
            fn unknown_email_is_none_not_an_error() {
                contract::unknown_email_is_none_not_an_error(|| $new_adapter);
            }

            #[test]
            fn unknown_username_is_none_not_an_error() {
                contract::unknown_username_is_none_not_an_error(|| $new_adapter);
            }

            #[test]
            fn find_by_email_matches_the_whole_email() {
                contract::find_by_email_matches_the_whole_email(|| $new_adapter);
            }

            #[test]
            fn find_by_username_matches_the_whole_username() {
                contract::find_by_username_matches_the_whole_username(|| $new_adapter);
            }

            #[test]
            fn finds_each_account_by_its_own_email_and_username() {
                contract::finds_each_account_by_its_own_email_and_username(|| $new_adapter);
            }

            #[test]
            fn save_replaces_the_account_with_the_same_id() {
                contract::save_replaces_the_account_with_the_same_id(|| $new_adapter);
            }
        }
    };
}

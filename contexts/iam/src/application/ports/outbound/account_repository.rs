use crate::domain::model::account::account::Account;
use shared::Fault;

/// Persistencia de cuentas. No declara errores (ERR-3): cualquier falla técnica
/// es `Fault` (árbol B). "No existe" no es un error: `find_*` devuelve `None`.
pub trait AccountRepository: Send + Sync {
    fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault>;

    fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault>;

    fn save(&self, account: &Account) -> Result<(), Fault>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::{
        application::ports::outbound::account_repository::AccountRepository,
        domain::model::account::{account::Account, account_factory::AccountFactory},
    };
    use shared::Fault;

    pub struct FakeAccountRepository {
        should_fail: bool,
        activated: bool,
        existing_username: Option<String>,
        existing_email: Option<String>,
    }

    impl FakeAccountRepository {
        pub fn success() -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: None,
                existing_email: None,
            }
        }

        pub fn fail() -> Self {
            Self {
                should_fail: true,
                activated: false,
                existing_username: None,
                existing_email: None,
            }
        }

        pub fn with_existing_email(email: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: None,
                existing_email: Some(email.to_string()),
            }
        }

        pub fn with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                activated: false,
                existing_username: Some(username.to_string()),
                existing_email: None,
            }
        }

        pub fn active_with_existing_username(username: &str) -> Self {
            Self {
                should_fail: false,
                activated: true,
                existing_username: Some(username.to_string()),
                existing_email: None,
            }
        }
    }

    impl AccountRepository for FakeAccountRepository {
        fn find_by_username(&self, username: &str) -> Result<Option<Account>, Fault> {
            Ok(self
                .existing_username
                .as_ref()
                .filter(|u| u.as_str() == username)
                .map(|_| {
                    if !self.activated {
                        AccountFactory::dummy_account()
                    } else {
                        AccountFactory::dummy_account_with_status("active".to_string())
                    }
                }))
        }

        fn find_by_email(&self, email: &str) -> Result<Option<Account>, Fault> {
            Ok(self
                .existing_email
                .as_ref()
                .filter(|e| e.as_str() == email)
                .map(|_| AccountFactory::dummy_account()))
        }

        fn save(&self, _user: &Account) -> Result<(), Fault> {
            if self.should_fail {
                Err(Fault::new("FakeAccountRepository error"))
            } else {
                Ok(())
            }
        }
    }
}

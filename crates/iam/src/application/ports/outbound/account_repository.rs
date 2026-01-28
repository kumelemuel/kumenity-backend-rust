use crate::domain::aggregates::Account;

pub trait AccountRepositoryPort: Send + Sync {
    fn find_by_username(&self, username: &str) -> Option<Account>;

    fn find_by_email(&self, email: &str) -> Option<Account>;
    fn save(&self, user: &Account) -> Result<(), String>;
}

#[cfg(test)]
pub mod test_utils {
    use crate::application::ports::outbound::account_repository::AccountRepositoryPort;
    use crate::domain::aggregates::Account;

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

    impl AccountRepositoryPort for FakeAccountRepository {
        fn find_by_username(&self, username: &str) -> Option<Account> {
            self.existing_username
                .as_ref()
                .filter(|u| u.as_str() == username)
                .map(|_| {
                    if !self.activated {
                        Account::dummy_account()
                    } else {
                        Account::dummy_active_account()
                    }
                })
        }

        fn find_by_email(&self, email: &str) -> Option<Account> {
            self.existing_email
                .as_ref()
                .filter(|e| e.as_str() == email)
                .map(|_| Account::dummy_account())
        }

        fn save(&self, _user: &Account) -> Result<(), String> {
            if self.should_fail {
                Err("Unexpected error".to_string())
            } else {
                Ok(())
            }
        }
    }
}
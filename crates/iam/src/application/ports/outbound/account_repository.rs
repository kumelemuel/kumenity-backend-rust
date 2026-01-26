use crate::domain::aggregates::Account;

pub trait AccountRepositoryPort: Send + Sync {
    fn find_by_username(&self, username: &str) -> Option<Account>;

    fn find_by_email(&self, email: &str) -> Option<Account>;
    fn save(&self, user: &Account) -> Result<(), String>;
}

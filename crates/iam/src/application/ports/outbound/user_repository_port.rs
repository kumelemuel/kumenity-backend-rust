use crate::domain::aggregates::User;

pub trait UserRepositoryPort: Send + Sync {
    fn find_by_username(&self, username: &str) -> Option<User>;

    fn find_by_email(&self, email: &str) -> Option<User>;
    fn save(&self, user: &User) -> Result<(), String>;
}


use crate::domain::value_objects::{Email, HashedPassword, UserId};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Email,
    password: HashedPassword,
    active: bool,
}

impl User {
    pub fn new(id: UserId, email: Email, password: HashedPassword) -> Self {
        Self {
            id,
            email,
            password,
            active: true,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn change_email(&mut self, new_email: Email) {
        self.email = new_email;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{UserId, Email, HashedPassword};

    #[test]
    fn create_user_successfully() {
        let id = UserId::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = HashedPassword::dummy();

        let user = User::new(id.clone(), email.clone(), password.clone());

        assert_eq!(user.id(), &id);
        assert_eq!(user.email(), &email);
        assert_eq!(user.password(), &password);
        assert!(user.is_active());
    }

    #[test]
    fn deactivate_user() {
        let id = UserId::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = HashedPassword::dummy();

        let mut user = User::new(id, email, password);
        assert!(user.is_active());

        user.deactivate();
        assert!(!user.is_active());
    }

    #[test]
    fn change_email() {
        let id = UserId::new();
        let email = Email::new("old@example.com".to_string()).unwrap();
        let password = HashedPassword::dummy();

        let mut user = User::new(id, email, password);

        let new_email = Email::new("new@example.com".to_string()).unwrap();
        user.change_email(new_email.clone());

        assert_eq!(user.email(), &new_email);
    }
}
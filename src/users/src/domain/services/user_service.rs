use crate::domain::entities::user::User;

use crate::domain::value_objects::{Email};

pub struct UserDomainService;

impl UserDomainService {
    pub fn change_email(user: &mut User, new_email: Email) -> Result<(), String> {
        user.change_email(new_email);
        Ok(())
    }

    pub fn activate_user(user: &mut User) {
        user.activate();
    }

    pub fn deactivate_user(user: &mut User) {
        user.deactivate();
    }

    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            Err("Password too short".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::User;
    use crate::domain::value_objects::{UserId, Email, HashedPassword};

    fn dummy_user() -> User {
        let id = UserId::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = HashedPassword::dummy();
        User::new(id, email, password)
    }

    #[test]
    fn change_email_works() {
        let mut user = dummy_user();
        let new_email = Email::new("new@example.com".to_string()).unwrap();
        UserDomainService::change_email(&mut user, new_email.clone()).unwrap();
        assert_eq!(user.email(), &new_email);
    }

    #[test]
    fn activate_and_deactivate_user() {
        let mut user = dummy_user();
        UserDomainService::deactivate_user(&mut user);
        assert!(!user.is_active());
        UserDomainService::activate_user(&mut user);
        assert!(user.is_active());
    }
}
use crate::domain::errors::account_id::AccountIdError;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Result<Self, AccountIdError> {
        if uuid.is_nil() {
            return Err(AccountIdError::Invalid);
        }

        Ok(Self(uuid))
    }

    pub fn from_str(value: &str) -> Result<Self, AccountIdError> {
        let uuid = Uuid::parse_str(value).map_err(|_| AccountIdError::WrongFormat)?;
        Self::from_uuid(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn generates_unique_ids() {
        let id1 = AccountId::generate();
        let id2 = AccountId::generate();

        assert_ne!(id1, id2);
    }

    #[test]
    fn creates_from_valid_uuid() {
        let uuid = Uuid::new_v4();
        let user_id = AccountId::from_uuid(uuid).unwrap();

        assert_eq!(user_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_nil_uuid() {
        let result = AccountId::from_uuid(Uuid::nil());

        assert!(matches!(result, Err(AccountIdError::Invalid)));
    }

    #[test]
    fn creates_from_valid_string() {
        let uuid = Uuid::new_v4();
        let user_id = AccountId::from_str(&uuid.to_string()).unwrap();

        assert_eq!(user_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_invalid_string() {
        let result = AccountId::from_str("not-a-uuid");

        assert!(matches!(result, Err(AccountIdError::WrongFormat)));
    }
}

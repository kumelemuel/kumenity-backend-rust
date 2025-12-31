use uuid::Uuid;

use crate::domain::errors::invalid_user_id::InvalidUserId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Result<Self, InvalidUserId> {
        if uuid.is_nil() {
            return Err(InvalidUserId);
        }

        Ok(Self(uuid))
    }

    pub fn from_str(value: &str) -> Result<Self, InvalidUserId> {
        let uuid = Uuid::parse_str(value).map_err(|_| InvalidUserId)?;
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
        let id1 = UserId::generate();
        let id2 = UserId::generate();

        assert_ne!(id1, id2);
    }

    #[test]
    fn creates_from_valid_uuid() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::from_uuid(uuid).unwrap();

        assert_eq!(user_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_nil_uuid() {
        let result = UserId::from_uuid(Uuid::nil());

        assert_eq!(
            result,
            Err(crate::domain::errors::invalid_user_id::InvalidUserId)
        );
    }

    #[test]
    fn creates_from_valid_string() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::from_str(&uuid.to_string()).unwrap();

        assert_eq!(user_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_invalid_string() {
        let result = UserId::from_str("not-a-uuid");

        assert_eq!(
            result,
            Err(crate::domain::errors::invalid_user_id::InvalidUserId)
        );
    }
}

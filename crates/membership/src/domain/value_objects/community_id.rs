use uuid::Uuid;
use crate::domain::errors::invalid_community_id::InvalidCommunityId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommunityId(Uuid);

impl CommunityId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Result<Self, InvalidCommunityId> {
        if uuid.is_nil() {
            return Err(InvalidCommunityId);
        }

        Ok(Self(uuid))
    }

    pub fn from_str(value: &str) -> Result<Self, InvalidCommunityId> {
        let uuid = Uuid::parse_str(value).map_err(|_| InvalidCommunityId)?;
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
        let id1 = CommunityId::generate();
        let id2 = CommunityId::generate();

        assert_ne!(id1, id2);
    }

    #[test]
    fn creates_from_valid_uuid() {
        let uuid = Uuid::new_v4();
        let community_id = CommunityId::from_uuid(uuid).unwrap();

        assert_eq!(community_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_nil_uuid() {
        let result = CommunityId::from_uuid(Uuid::nil());

        assert_eq!(
            result,
            Err(InvalidCommunityId)
        );
    }

    #[test]
    fn creates_from_valid_string() {
        let uuid = Uuid::new_v4();
        let community_id = CommunityId::from_str(&uuid.to_string()).unwrap();

        assert_eq!(community_id.as_uuid(), &uuid);
    }

    #[test]
    fn rejects_invalid_string() {
        let result = CommunityId::from_str("not-a-uuid");

        assert_eq!(
            result,
            Err(InvalidCommunityId)
        );
    }
}

use crate::domain::errors::invalid_owner_id::InvalidOwnerId;
use uuid::Uuid;

/// Who owns a community, in this context's own terms (HEX-1). It is the
/// subject of the verified identity; `communities` does not know `iam`'s
/// `AccountId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnerId(Uuid);

impl OwnerId {
    // Pendiente: implementar `FromStr` en lugar de un método inherente (cambia los llamadores).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Result<Self, InvalidOwnerId> {
        let uuid = Uuid::parse_str(value).map_err(|_| InvalidOwnerId::WrongFormat)?;
        if uuid.is_nil() {
            return Err(InvalidOwnerId::Nil);
        }
        Ok(Self(uuid))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60";

    #[test]
    fn owner_id_accepts_a_uuid() {
        let owner_id = OwnerId::from_str(VALID).expect("a non-nil UUID is an owner id");

        assert_eq!(owner_id.as_uuid().to_string(), VALID);
    }

    #[test]
    fn owner_id_rejects_non_uuid() {
        assert_eq!(
            OwnerId::from_str("not-a-uuid"),
            Err(InvalidOwnerId::WrongFormat)
        );
    }

    #[test]
    fn owner_id_rejects_nil() {
        assert_eq!(
            OwnerId::from_str("00000000-0000-0000-0000-000000000000"),
            Err(InvalidOwnerId::Nil)
        );
    }
}

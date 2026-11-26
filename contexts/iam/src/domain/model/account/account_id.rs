use uuid::Uuid;

/// The id of an account, as the `IdGenerator` gives it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_unique_ids() {
        let id1 = AccountId::generate();
        let id2 = AccountId::generate();

        assert_ne!(id1, id2);
    }

    #[test]
    fn keeps_the_given_value() {
        let id = AccountId::new("acc-1".to_string());

        assert_eq!(id.as_str(), "acc-1");
    }
}

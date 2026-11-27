/// The id of an account, as the `IdGenerator` gives it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_given_value() {
        let id = AccountId::new("acc-1".to_string());

        assert_eq!(id.as_str(), "acc-1");
    }
}

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCommunityId;

impl fmt::Display for InvalidCommunityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community id")
    }
}

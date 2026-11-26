use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidCommunityName;

impl fmt::Display for InvalidCommunityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid community name")
    }
}

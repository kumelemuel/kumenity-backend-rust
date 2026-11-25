/// Leaf (ERR-2): something the use case requires does not exist. `resource`
/// names what is missing (e.g. `"account"`), never the value looked up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotFound {
    pub resource: &'static str,
}

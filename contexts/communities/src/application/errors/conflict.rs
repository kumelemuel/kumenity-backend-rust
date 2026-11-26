/// Leaf (ERR-2): the input collides with something that already exists.
/// `resource` names what is taken (e.g. `"email"`), never the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conflict {
    pub resource: &'static str,
}

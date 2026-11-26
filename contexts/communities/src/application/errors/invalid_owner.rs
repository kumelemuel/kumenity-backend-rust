/// Leaf (ERR-2): the subject of the verified identity is not a valid owner
/// (D2 a). It carries why, by variant, because the edge answers a different
/// code for each: the nil UUID or something that is not a UUID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidOwner {
    Nil,
    WrongFormat,
}

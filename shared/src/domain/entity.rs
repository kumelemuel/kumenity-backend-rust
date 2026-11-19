pub trait Entity {
    type Id: PartialEq + Clone;

    fn id(&self) -> &Self::Id;

    fn same_identity_as(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

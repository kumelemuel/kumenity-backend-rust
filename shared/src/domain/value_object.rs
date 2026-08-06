pub trait ValueObject: Sized + Clone + PartialEq {
    type Value;
    type Error;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Value;
}

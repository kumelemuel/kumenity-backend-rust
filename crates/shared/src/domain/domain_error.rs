pub trait DomainError: std::fmt::Debug + std::fmt::Display {
    fn code(&self) -> &'static str;
}

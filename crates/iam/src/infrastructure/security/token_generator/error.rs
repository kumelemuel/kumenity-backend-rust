#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    Expired,
}

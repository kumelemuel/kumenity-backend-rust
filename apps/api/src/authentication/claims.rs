use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

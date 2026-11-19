use crate::authentication::{claims::Claims, error::TokenValidationError};
use iam::infrastructure::security::token_generator::{
    error::JwtError, jwt_token_generator::JwtTokenGenerator,
};
use std::sync::Arc;

pub trait TokenValidator: Send + Sync {
    fn validate(&self, token: &str) -> Result<Claims, TokenValidationError>;
}

pub struct JwtValidator {
    jwt: Arc<JwtTokenGenerator>,
}

impl JwtValidator {
    pub fn new(jwt: Arc<JwtTokenGenerator>) -> Self {
        Self { jwt }
    }
}

impl TokenValidator for JwtValidator {
    fn validate(&self, token: &str) -> Result<Claims, TokenValidationError> {
        let iam_claims = self.jwt.decode(token).map_err(|err| match err {
            JwtError::Expired => TokenValidationError::Expired,
            JwtError::InvalidToken => TokenValidationError::Invalid,
        })?;

        Ok(Claims {
            sub: iam_claims.sub,
            _exp: iam_claims.exp,
        })
    }
}

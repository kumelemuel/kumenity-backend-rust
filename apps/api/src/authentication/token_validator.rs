use std::sync::Arc;
use iam::infrastructure::security::token_generator::error::JwtError;
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;
use crate::authentication::claims::Claims;
use crate::authentication::error::TokenValidationError;

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
            exp: iam_claims.exp,
        })
    }
}

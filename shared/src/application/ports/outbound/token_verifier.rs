use crate::{
    application::ports::outbound::error::TokenError,
    domain::model::verified_identity::VerifiedIdentity,
};

pub trait TokenVerifier: Send + Sync {
    fn verify(&self, token: &str) -> Result<VerifiedIdentity, TokenError>;
}

use crate::application::queries::verify_access_token::{
    VerifyAccessTokenError, VerifyAccessTokenQuery, VerifyAccessTokenResponse,
};
use shared::Failure;

/// Verifies an access token and answers the account it was issued to
/// (HEX-4). The authentication middleware calls it for the protected routes
/// of every context.
pub trait VerifyAccessTokenUseCase: Send + Sync {
    fn execute(
        &self,
        query: VerifyAccessTokenQuery,
    ) -> Result<VerifyAccessTokenResponse, Failure<VerifyAccessTokenError>>;
}

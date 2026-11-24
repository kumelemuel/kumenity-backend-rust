use super::{VerifyAccessTokenError, VerifyAccessTokenQuery, VerifyAccessTokenResponse};
use crate::application::{
    errors::unauthenticated::Unauthenticated,
    ports::{
        inbound::verify_access_token_use_case::VerifyAccessTokenUseCase,
        outbound::{
            access_token_service::AccessTokenService,
            dtos::access_token_payloads::AccessTokenVerdict,
        },
    },
};
use shared::Failure;
use std::sync::Arc;

/// Orchestrates (HEX-7): asks for the verdict and maps it.
pub struct VerifyAccessTokenHandler {
    access_tokens: Arc<dyn AccessTokenService>,
}

impl VerifyAccessTokenHandler {
    pub fn new(access_tokens: Arc<dyn AccessTokenService>) -> Self {
        Self { access_tokens }
    }
}

impl VerifyAccessTokenUseCase for VerifyAccessTokenHandler {
    fn execute(
        &self,
        query: VerifyAccessTokenQuery,
    ) -> Result<VerifyAccessTokenResponse, Failure<VerifyAccessTokenError>> {
        // The `Fault` is discarded on purpose, keeping today's 401, until the
        // commit that propagates it with its test (R10b).
        let verdict = self
            .access_tokens
            .verify(&query.token)
            .map_err(|_fault| Failure::expected(Unauthenticated))?;

        // `Invalid` is a success of the port that this use case treats as an
        // error (ERR-3). The account id is not validated here: the contexts
        // that use it decide, as `communities` does with its 400.
        match verdict {
            AccessTokenVerdict::Valid { account_id } => {
                Ok(VerifyAccessTokenResponse { account_id })
            }
            AccessTokenVerdict::Invalid => Err(Failure::expected(Unauthenticated)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::testing::FakeAccessTokenAdapter;

    const ACCOUNT_ID: &str = "6f1c1b2e-3d4a-4b5c-8d9e-0f1a2b3c4d5e";

    fn verify(
        access_tokens: Arc<FakeAccessTokenAdapter>,
        token: &str,
    ) -> Result<VerifyAccessTokenResponse, Failure<VerifyAccessTokenError>> {
        VerifyAccessTokenHandler::new(access_tokens).execute(VerifyAccessTokenQuery {
            token: token.to_string(),
        })
    }

    #[test]
    fn returns_account_id_for_valid_token() {
        let access_tokens = Arc::new(FakeAccessTokenAdapter::new());
        let token = access_tokens.issue(ACCOUNT_ID).expect("issue");

        let result = verify(access_tokens, &token);

        assert_eq!(
            result.ok(),
            Some(VerifyAccessTokenResponse {
                account_id: ACCOUNT_ID.to_string()
            })
        );
    }

    #[test]
    fn fails_with_unauthenticated_when_token_is_invalid() {
        let result = verify(Arc::new(FakeAccessTokenAdapter::new()), "not-a-token");

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccessTokenError::Unauthenticated(
                    Unauthenticated
                )))
            ),
            "{result:?}"
        );
    }

    #[test]
    fn fails_with_unauthenticated_when_token_is_expired() {
        let access_tokens = Arc::new(FakeAccessTokenAdapter::new());
        let token = access_tokens.issue_expired(ACCOUNT_ID);

        let result = verify(access_tokens, &token);

        assert!(
            matches!(
                result,
                Err(Failure::Expected(VerifyAccessTokenError::Unauthenticated(
                    Unauthenticated
                )))
            ),
            "{result:?}"
        );
    }
}

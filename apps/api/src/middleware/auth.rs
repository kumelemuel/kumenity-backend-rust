use crate::authentication::{
    bearer::extract_bearer, error::AuthError, token_validator::TokenValidator,
};
use shared::application::auth_context::AuthContext;
use std::sync::Arc;

pub fn authenticate(
    headers: &http::HeaderMap,
    validator: Arc<dyn TokenValidator>,
) -> Result<AuthContext, AuthError> {
    let token = extract_bearer(headers)?;
    let claims = validator.validate(&token)?;

    Ok(AuthContext {
        account_id: claims.sub,
    })
}

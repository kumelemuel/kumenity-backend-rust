use std::sync::Arc;
use shared::application::auth_context::AuthContext;
use crate::authentication::bearer::extract_bearer;
use crate::authentication::error::AuthError;
use crate::authentication::token_validator::TokenValidator;

pub fn authenticate(headers: &http::HeaderMap, validator: Arc<dyn TokenValidator>,) -> Result<AuthContext, AuthError> {
    let token = extract_bearer(headers)?;
    let claims = validator.validate(&token)?;

    Ok(AuthContext {
        account_id: claims.sub,
    })
}

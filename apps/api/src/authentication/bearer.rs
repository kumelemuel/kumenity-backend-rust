use http::HeaderMap;

#[derive(Debug)]
pub enum AuthHeaderError {
    Missing,
    InvalidFormat,
}

pub fn extract_bearer(headers: &HeaderMap) -> Result<String, AuthHeaderError> {
    let value = headers
        .get("authorization")
        .ok_or(AuthHeaderError::Missing)?;

    let value = value.to_str().map_err(|_| AuthHeaderError::InvalidFormat)?;

    let (scheme, token) = value
        .split_once(' ')
        .ok_or(AuthHeaderError::InvalidFormat)?;

    if scheme != "Bearer" {
        return Err(AuthHeaderError::InvalidFormat);
    }

    Ok(token.to_owned())
}

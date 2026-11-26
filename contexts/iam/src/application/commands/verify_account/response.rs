/// Verifying an account returns no data (D11): the type exists so the use
/// case answers its own contract. The HTTP edge does not serialize it.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifyAccountResponse;

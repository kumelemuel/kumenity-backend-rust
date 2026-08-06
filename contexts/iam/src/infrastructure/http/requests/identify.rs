use crate::application::queries::identify_account::query::IdentifyAccountQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IdentifyRequest {
    pub identify: String,
}

impl From<IdentifyRequest> for IdentifyAccountQuery {
    fn from(req: IdentifyRequest) -> Self {
        IdentifyAccountQuery {
            identify: req.identify,
        }
    }
}

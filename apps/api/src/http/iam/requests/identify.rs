
use serde::Deserialize;
use iam::application::commands::identify_account::IdentifyAccount;

#[derive(Debug, Deserialize)]
pub struct IdentifyRequest {
    pub identify: String,
}

impl From<IdentifyRequest> for IdentifyAccount {
    fn from(req: IdentifyRequest) -> Self {
        IdentifyAccount {
            identify: req.identify,
        }
    }
}

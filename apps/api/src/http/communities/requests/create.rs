
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
}

// impl From<CreateRequest> for IdentifyAccount {
//     fn from(req: CreateRequest) -> Self {
//         IdentifyAccount {
//             identify: req.identify,
//         }
//     }
// }

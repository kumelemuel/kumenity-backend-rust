use crate::application::queries::identify_account::{
    query::IdentifyAccountQuery, response::IdentifyAccountResponse,
};
use shared::error::SystemError;

pub trait IdentifyAccountUseCase: Send + Sync {
    fn execute(&self, query: IdentifyAccountQuery) -> Result<IdentifyAccountResponse, SystemError>;
}

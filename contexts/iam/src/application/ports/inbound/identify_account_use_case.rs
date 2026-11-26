use crate::application::queries::identify_account::{
    IdentifyAccountError, IdentifyAccountQuery, IdentifyAccountResponse,
};
use shared::Failure;

pub trait IdentifyAccountUseCase: Send + Sync {
    fn execute(
        &self,
        query: IdentifyAccountQuery,
    ) -> Result<IdentifyAccountResponse, Failure<IdentifyAccountError>>;
}

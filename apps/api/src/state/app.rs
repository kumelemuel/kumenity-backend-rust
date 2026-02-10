use std::sync::Arc;
use crate::authentication::token_validator::TokenValidator;
use crate::state::communities::CommunitiesState;
use crate::state::iam::IamState;

#[derive(Clone)]
pub struct AppState {
    pub iam: IamState,
    pub communities: CommunitiesState,
    pub token_validator: Arc<dyn TokenValidator>,
}

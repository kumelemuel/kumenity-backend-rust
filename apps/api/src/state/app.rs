use crate::{
    authentication::token_validator::TokenValidator,
    state::{communities::CommunitiesState, iam::IamState},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub iam: IamState,
    pub communities: CommunitiesState,
    pub token_validator: Arc<dyn TokenValidator>,
}

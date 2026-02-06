use std::sync::Arc;
use communities::application::ports::inbound::community_creation::CommunityCreationPort;
use iam::application::ports::inbound::account_registration::AccountRegistrationPort;
use iam::application::ports::inbound::account_authentication::AccountAuthenticationPort;
use iam::application::ports::inbound::account_identification::AccountIdentificationPort;
use iam::application::ports::inbound::account_verification::AccountVerificationPort;
use crate::authentication::token_validator::TokenValidator;

#[derive(Clone)]
pub struct AppState {
    pub register_account: Arc<dyn AccountRegistrationPort + Send + Sync>,
    pub authenticate_account: Arc<dyn AccountAuthenticationPort + Send + Sync>,
    pub verify_account: Arc<dyn AccountVerificationPort + Send + Sync>,
    pub identify_account: Arc<dyn AccountIdentificationPort + Send + Sync>,
    pub token_validator: Arc<dyn TokenValidator>,
    pub create_community: Arc<dyn CommunityCreationPort + Send + Sync>,
}

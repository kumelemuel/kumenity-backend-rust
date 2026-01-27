use std::sync::Arc;
use iam::application::ports::inbound::account_registration::AccountRegistrationPort;
use iam::application::ports::inbound::account_authentication::AccountAuthenticationPort;
use iam::application::ports::inbound::account_identification::AccountIdentificationPort;
use iam::application::ports::inbound::account_verification::AccountVerificationPort;

#[derive(Clone)]
pub struct AppState {
    pub register_account: Arc<dyn AccountRegistrationPort + Send + Sync>,
    pub authenticate_account: Arc<dyn AccountAuthenticationPort + Send + Sync>,
    pub verify_account: Arc<dyn AccountVerificationPort + Send + Sync>,
    pub identify_account: Arc<dyn AccountIdentificationPort + Send + Sync>,
}

use std::sync::Arc;
use iam::application::ports::inbound::account_registration::AccountRegistrationPort;
use iam::application::ports::inbound::account_authentication::AccountAuthenticationPort;
use iam::application::ports::inbound::account_verification::AccountVerificationPort;

#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<dyn AccountRegistrationPort + Send + Sync>,
    pub login: Arc<dyn AccountAuthenticationPort + Send + Sync>,
    pub validate_user: Arc<dyn AccountVerificationPort + Send + Sync>,
}

use std::sync::Arc;
use iam::application::ports::inbound::register_user_port::RegisterUserPort;
use iam::application::ports::inbound::login_port::LoginPort;
use iam::application::ports::inbound::validate_user_port::ValidateUserPort;

#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<dyn RegisterUserPort + Send + Sync>,
    pub login: Arc<dyn LoginPort + Send + Sync>,
    pub validate_user: Arc<dyn ValidateUserPort + Send + Sync>,
}

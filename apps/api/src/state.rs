use std::sync::Arc;
use iam::application::ports::inbound::register_user_port::RegisterUserPort;
use iam::application::ports::inbound::login_port::LoginPort;

#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<dyn RegisterUserPort + Send + Sync>,
    pub login: Arc<dyn LoginPort + Send + Sync>,
}

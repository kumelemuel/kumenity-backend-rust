use iam::application::ports::inbound::register_user_port::RegisterUserPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<dyn RegisterUserPort + Send + Sync>,
}

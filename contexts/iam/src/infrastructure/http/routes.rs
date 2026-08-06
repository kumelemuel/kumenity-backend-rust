use crate::{
    application::ports::inbound::{
        authenticate_account_use_case::AuthenticateAccountUseCase,
        identify_account_use_case::IdentifyAccountUseCase,
        register_account_use_case::RegisterAccountUseCase,
        verify_account_use_case::VerifyAccountUseCase,
    },
    infrastructure::http::handlers::account_handlers,
};
use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

pub struct IamRoutes {
    register: Arc<dyn RegisterAccountUseCase>,
    authenticate: Arc<dyn AuthenticateAccountUseCase>,
    verify: Arc<dyn VerifyAccountUseCase>,
    identify: Arc<dyn IdentifyAccountUseCase>,
}

impl IamRoutes {
    pub fn new(
        register: Arc<dyn RegisterAccountUseCase>,
        authenticate: Arc<dyn AuthenticateAccountUseCase>,
        verify: Arc<dyn VerifyAccountUseCase>,
        identify: Arc<dyn IdentifyAccountUseCase>,
    ) -> Self {
        Self {
            register,
            authenticate,
            verify,
            identify,
        }
    }

    pub fn public(&self) -> Router {
        Router::new()
            .route(
                "/auth/register",
                post(account_handlers::register).with_state(self.register.clone()),
            )
            .route(
                "/auth/authenticate",
                post(account_handlers::authenticate).with_state(self.authenticate.clone()),
            )
            .route(
                "/auth/verify",
                post(account_handlers::verify).with_state(self.verify.clone()),
            )
            .route(
                "/auth/identify",
                post(account_handlers::identify).with_state(self.identify.clone()),
            )
    }

    pub fn protected(&self) -> Router {
        Router::new()
    }
}

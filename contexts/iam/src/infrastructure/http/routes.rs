use crate::{
    application::ports::inbound::{
        authenticate_account_use_case::AuthenticateAccountUseCase,
        identify_account_use_case::IdentifyAccountUseCase,
        register_account_use_case::RegisterAccountUseCase,
        verify_account_use_case::VerifyAccountUseCase,
    },
    infrastructure::http::{
        authenticate_account, identify_account, register_account, verify_account,
    },
};
use axum::{Router, routing::post};
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
                post(register_account::register).with_state(self.register.clone()),
            )
            .route(
                "/auth/authenticate",
                post(authenticate_account::authenticate).with_state(self.authenticate.clone()),
            )
            .route(
                "/auth/verify",
                post(verify_account::verify).with_state(self.verify.clone()),
            )
            .route(
                "/auth/identify",
                post(identify_account::identify).with_state(self.identify.clone()),
            )
    }

    pub fn protected(&self) -> Router {
        Router::new()
    }
}

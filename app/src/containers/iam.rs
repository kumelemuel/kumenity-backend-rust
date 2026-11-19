use iam::{
    application::{
        commands::{
            authenticate_account::authenticate_account::AuthenticateAccount,
            register_account::register_account::RegisterAccount,
            verify_account::verify_account::VerifyAccount,
        },
        ports::inbound::{
            authenticate_account_use_case::AuthenticateAccountUseCase,
            identify_account_use_case::IdentifyAccountUseCase,
            register_account_use_case::RegisterAccountUseCase,
            verify_account_use_case::VerifyAccountUseCase,
        },
        queries::identify_account::identify_account::IdentifyAccount,
    },
    infrastructure::{
        http::routes::IamRoutes,
        persistence::in_memory::account_repository::InMemoryAccountRepository,
        security::password_hasher::argon2_password_hasher::Argon2PasswordHasher,
        services::jwt_service::jwt_service::JwtService,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct IamContainer {
    pub register_account: Arc<dyn RegisterAccountUseCase + Send + Sync>,
    pub authenticate_account: Arc<dyn AuthenticateAccountUseCase + Send + Sync>,
    pub verify_account: Arc<dyn VerifyAccountUseCase + Send + Sync>,
    pub identify_account: Arc<dyn IdentifyAccountUseCase + Send + Sync>,
}

impl IamContainer {
    pub fn initialize(jwt_service: Arc<JwtService>) -> Self {
        let account_repository = Arc::new(InMemoryAccountRepository::new());
        let password_hasher = Arc::new(Argon2PasswordHasher::new());

        let register_account =
            RegisterAccount::new(account_repository.clone(), password_hasher.clone());
        let authenticate_account = AuthenticateAccount::new(
            account_repository.clone(),
            password_hasher.clone(),
            jwt_service,
        );
        let verify_account = VerifyAccount::new(account_repository.clone());
        let identify_account = IdentifyAccount::new(account_repository.clone());

        Self {
            register_account: Arc::new(register_account),
            authenticate_account: Arc::new(authenticate_account),
            verify_account: Arc::new(verify_account),
            identify_account: Arc::new(identify_account),
        }
    }

    pub fn routes(&self) -> IamRoutes {
        IamRoutes::new(
            self.register_account.clone(),
            self.authenticate_account.clone(),
            self.verify_account.clone(),
            self.identify_account.clone(),
        )
    }
}

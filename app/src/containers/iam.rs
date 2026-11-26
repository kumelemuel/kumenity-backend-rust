use iam::{
    application::{
        commands::{
            authenticate_account::AuthenticateAccountHandler,
            register_account::RegisterAccountHandler, verify_account::VerifyAccountHandler,
        },
        ports::{
            inbound::{
                authenticate_account_use_case::AuthenticateAccountUseCase,
                identify_account_use_case::IdentifyAccountUseCase,
                register_account_use_case::RegisterAccountUseCase,
                verify_access_token_use_case::VerifyAccessTokenUseCase,
                verify_account_use_case::VerifyAccountUseCase,
            },
            outbound::password_hashing_service::PasswordHashingService,
        },
        queries::{
            identify_account::IdentifyAccountHandler, verify_access_token::VerifyAccessTokenHandler,
        },
    },
    infrastructure::{
        http::routes::IamRoutes, persistence::in_memory::in_memory_account::InMemoryAccountAdapter,
        security::argon2_password_hashing::Argon2PasswordHashingAdapter,
        services::jwt_access_token::JwtAccessTokenAdapter,
    },
};
use shared::Fault;
use std::{
    hash::{BuildHasher, Hasher, RandomState},
    sync::Arc,
};

#[derive(Clone)]
pub struct IamContainer {
    pub register_account: Arc<dyn RegisterAccountUseCase + Send + Sync>,
    pub authenticate_account: Arc<dyn AuthenticateAccountUseCase + Send + Sync>,
    pub verify_account: Arc<dyn VerifyAccountUseCase + Send + Sync>,
    pub identify_account: Arc<dyn IdentifyAccountUseCase + Send + Sync>,
    pub verify_access_token: Arc<dyn VerifyAccessTokenUseCase>,
}

impl IamContainer {
    /// Fails if the decoy hash cannot be generated: without it, sign-in would
    /// reveal which usernames exist (D6), so the server does not start.
    pub fn initialize(access_tokens: Arc<JwtAccessTokenAdapter>) -> Result<Self, Fault> {
        let account_repository = Arc::new(InMemoryAccountAdapter::new());
        let password_hasher = Arc::new(Argon2PasswordHashingAdapter::new());

        let register_account =
            RegisterAccountHandler::new(account_repository.clone(), password_hasher.clone());
        // Hashed once, with the same adapter and parameters as real
        // passwords, so verifying against it costs the same (D6).
        let decoy_hash = password_hasher.hash(&random_decoy_password())?;
        let authenticate_account = AuthenticateAccountHandler::new(
            account_repository.clone(),
            password_hasher.clone(),
            access_tokens.clone(),
            decoy_hash,
        );
        let verify_account = VerifyAccountHandler::new(account_repository.clone());
        let identify_account = IdentifyAccountHandler::new(account_repository.clone());
        let verify_access_token = VerifyAccessTokenHandler::new(access_tokens);

        Ok(Self {
            register_account: Arc::new(register_account),
            authenticate_account: Arc::new(authenticate_account),
            verify_account: Arc::new(verify_account),
            identify_account: Arc::new(identify_account),
            verify_access_token: Arc::new(verify_access_token),
        })
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

fn random_decoy_password() -> String {
    format!("{:016x}", RandomState::new().build_hasher().finish())
}

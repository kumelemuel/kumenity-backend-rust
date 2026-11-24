use iam::{
    application::{
        commands::{
            authenticate_account::authenticate_account::AuthenticateAccount,
            register_account::register_account::RegisterAccount,
            verify_account::verify_account::VerifyAccount,
        },
        ports::{
            inbound::{
                authenticate_account_use_case::AuthenticateAccountUseCase,
                identify_account_use_case::IdentifyAccountUseCase,
                register_account_use_case::RegisterAccountUseCase,
                verify_account_use_case::VerifyAccountUseCase,
            },
            outbound::password_hashing_service::PasswordHashingService,
        },
        queries::identify_account::identify_account::IdentifyAccount,
    },
    infrastructure::{
        http::routes::IamRoutes, persistence::in_memory::in_memory_account::InMemoryAccountAdapter,
        security::password_hasher::argon2_password_hasher::Argon2PasswordHasher,
        services::jwt_service::jwt_service::JwtService,
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
}

impl IamContainer {
    /// Fails if the decoy hash cannot be generated: without it, sign-in would
    /// reveal which usernames exist (D6), so the server does not start.
    pub fn initialize(jwt_service: Arc<JwtService>) -> Result<Self, Fault> {
        let account_repository = Arc::new(InMemoryAccountAdapter::new());
        let password_hasher = Arc::new(Argon2PasswordHasher::new());

        let register_account =
            RegisterAccount::new(account_repository.clone(), password_hasher.clone());
        // Hashed once, with the same adapter and parameters as real
        // passwords, so verifying against it costs the same (D6).
        let decoy_hash = password_hasher.hash(&random_decoy_password())?;
        let authenticate_account = AuthenticateAccount::new(
            account_repository.clone(),
            password_hasher.clone(),
            jwt_service,
            decoy_hash,
        );
        let verify_account = VerifyAccount::new(account_repository.clone());
        let identify_account = IdentifyAccount::new(account_repository.clone());

        Ok(Self {
            register_account: Arc::new(register_account),
            authenticate_account: Arc::new(authenticate_account),
            verify_account: Arc::new(verify_account),
            identify_account: Arc::new(identify_account),
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

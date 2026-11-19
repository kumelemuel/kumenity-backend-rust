use iam::{
    application::{
        ports::inbound::{
            account_authentication::AccountAuthenticationPort,
            account_identification::AccountIdentificationPort,
            account_registration::AccountRegistrationPort,
            account_verification::AccountVerificationPort,
        },
        use_cases::{
            authenticate_account::AuthenticateAccountUseCase,
            identify_account::IdentifyAccountUseCase, register_account::RegisterAccountUseCase,
            verify_account::VerifyAccountUseCase,
        },
    },
    infrastructure::{
        persistence::in_memory::account_repository::InMemoryAccountRepository,
        security::{
            password_hasher::argon2_password_hasher::Argon2PasswordHasher,
            token_generator::jwt_token_generator::JwtTokenGenerator,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct IamState {
    pub register_account: Arc<dyn AccountRegistrationPort + Send + Sync>,
    pub authenticate_account: Arc<dyn AccountAuthenticationPort + Send + Sync>,
    pub verify_account: Arc<dyn AccountVerificationPort + Send + Sync>,
    pub identify_account: Arc<dyn AccountIdentificationPort + Send + Sync>,
}

impl IamState {
    pub fn initialize(token_generator: Arc<JwtTokenGenerator>) -> Self {
        let account_repository = Arc::new(InMemoryAccountRepository::new());
        let password_hasher = Arc::new(Argon2PasswordHasher::new());

        let register_account =
            RegisterAccountUseCase::new(account_repository.clone(), password_hasher.clone());
        let authenticate_account = AuthenticateAccountUseCase::new(
            account_repository.clone(),
            password_hasher.clone(),
            token_generator,
        );
        let verify_account = VerifyAccountUseCase::new(account_repository.clone());
        let identify_account = IdentifyAccountUseCase::new(account_repository.clone());

        Self {
            register_account: Arc::new(register_account),
            authenticate_account: Arc::new(authenticate_account),
            verify_account: Arc::new(verify_account),
            identify_account: Arc::new(identify_account),
        }
    }
}

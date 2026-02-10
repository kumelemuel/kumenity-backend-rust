use std::sync::Arc;
use iam::application::ports::inbound::account_authentication::AccountAuthenticationPort;
use iam::application::ports::inbound::account_identification::AccountIdentificationPort;
use iam::application::ports::inbound::account_registration::AccountRegistrationPort;
use iam::application::ports::inbound::account_verification::AccountVerificationPort;
use iam::application::use_cases::authenticate_account::AuthenticateAccountUseCase;
use iam::application::use_cases::identify_account::IdentifyAccountUseCase;
use iam::application::use_cases::register_account::RegisterAccountUseCase;
use iam::application::use_cases::verify_account::VerifyAccountUseCase;
use iam::infrastructure::persistence::in_memory::account_repository::InMemoryAccountRepository;
use iam::infrastructure::security::password_hasher::argon2_password_hasher::Argon2PasswordHasher;
use iam::infrastructure::security::token_generator::jwt_token_generator::JwtTokenGenerator;

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

        let register_account = RegisterAccountUseCase::new(account_repository.clone(), password_hasher.clone());
        let authenticate_account = AuthenticateAccountUseCase::new(account_repository.clone(), password_hasher.clone(), token_generator);
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
use crate::{
    application::{
        commands::create_community::{CreateCommunityCommand, CreateCommunityError},
        errors::{conflict::Conflict, invalid_owner::InvalidOwner, validation::Validation},
        ports::inbound::create_community_use_case::CreateCommunityUseCase,
    },
    infrastructure::http::{
        codes, requests::create::CreateRequest, responses::created::CreatedResponse,
        unexpected::unexpected,
    },
};
use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{
    Failure, Fault, domain::model::verified_identity::VerifiedIdentity,
    infrastructure::http::api_error_response::ApiErrorResponse,
};
use std::sync::Arc;

/// `POST /communities/create`.
pub async fn create(
    State(use_case): State<Arc<dyn CreateCommunityUseCase>>,
    Extension(identity): Extension<VerifiedIdentity>,
    Json(request): Json<CreateRequest>,
) -> Response {
    // The caller's identity goes inside the command (HEX-4).
    let command = CreateCommunityCommand {
        owner_id: identity.account_id,
        slug: request.slug,
        name: request.name,
        is_public: request.is_public,
    };

    // Exhaustive `fold` of the contract into the transport (ERR-4.5).
    match use_case.execute(command) {
        Ok(result) => (StatusCode::OK, Json(CreatedResponse::from(result))).into_response(),
        Err(Failure::Expected(error)) => match error {
            CreateCommunityError::Validation(leaf) => validation(leaf),
            CreateCommunityError::Conflict(leaf) => conflict(leaf),
            CreateCommunityError::InvalidOwner(leaf) => invalid_owner(leaf),
        },
        Err(Failure::Unexpected(fault)) => unexpected(&fault),
    }
}

/// One helper per leaf (ERR-4). A field or resource the handler does not
/// emit is a bug, not a client error: tree B.
fn validation(leaf: Validation) -> Response {
    let (code, message) = match leaf.field {
        "slug" => (
            codes::COMMUNITIES_INVALID_COMMUNITY_SLUG,
            codes::COMMUNITIES_INVALID_COMMUNITY_SLUG_MESSAGE,
        ),
        "name" => (
            codes::COMMUNITIES_INVALID_COMMUNITY_NAME,
            codes::COMMUNITIES_INVALID_COMMUNITY_NAME_MESSAGE,
        ),
        field => return unexpected(&unmapped("validation field", field)),
    };
    error(StatusCode::BAD_REQUEST, code, message)
}

fn conflict(leaf: Conflict) -> Response {
    let (code, message) = match leaf.resource {
        "slug" => (
            codes::COMMUNITIES_SLUG_ALREADY_EXISTS,
            codes::COMMUNITIES_SLUG_ALREADY_EXISTS_MESSAGE,
        ),
        resource => return unexpected(&unmapped("conflict resource", resource)),
    };
    error(StatusCode::CONFLICT, code, message)
}

/// D2 a): the `IAM_` codes of today, one per reason.
fn invalid_owner(leaf: InvalidOwner) -> Response {
    let (code, message) = match leaf {
        InvalidOwner::Nil => (
            codes::IAM_INVALID_ACCOUNT_ID,
            codes::IAM_INVALID_ACCOUNT_ID_MESSAGE,
        ),
        InvalidOwner::WrongFormat => (
            codes::IAM_INVALID_ACCOUNT_ID_FORMAT,
            codes::IAM_INVALID_ACCOUNT_ID_FORMAT_MESSAGE,
        ),
    };
    error(StatusCode::BAD_REQUEST, code, message)
}

fn unmapped(kind: &str, name: &str) -> Fault {
    Fault::new(format!("create community: unmapped {kind} `{name}`"))
}

/// Body `{code, message}` of today (until Problem Details, R30).
fn error(status: StatusCode, code: &str, message: &str) -> Response {
    let body = ApiErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    };
    (status, Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::commands::create_community::CreateCommunityResponse,
        infrastructure::http::testing::{D7_BODY, body_of, ready},
    };
    use std::sync::Mutex;

    const OWNER: &str = "6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60";

    /// Always answers the same failure.
    struct StubCreateCommunityUseCase(fn() -> Failure<CreateCommunityError>);

    impl CreateCommunityUseCase for StubCreateCommunityUseCase {
        fn execute(
            &self,
            _command: CreateCommunityCommand,
        ) -> Result<CreateCommunityResponse, Failure<CreateCommunityError>> {
            Err((self.0)())
        }
    }

    /// Records the owner of the command it receives and creates it.
    #[derive(Default)]
    struct SpyCreateCommunityUseCase {
        owner_ids: Mutex<Vec<String>>,
    }

    impl CreateCommunityUseCase for SpyCreateCommunityUseCase {
        fn execute(
            &self,
            command: CreateCommunityCommand,
        ) -> Result<CreateCommunityResponse, Failure<CreateCommunityError>> {
            self.owner_ids
                .lock()
                .expect("invariante: mutex sin envenenar")
                .push(command.owner_id);
            Ok(CreateCommunityResponse {
                id: "c0ffee00-0000-4000-8000-000000000000".to_string(),
                name: command.name,
                slug: command.slug,
            })
        }
    }

    fn call(use_case: Arc<dyn CreateCommunityUseCase>) -> Response {
        let identity = VerifiedIdentity {
            account_id: OWNER.to_string(),
        };
        let request = CreateRequest {
            slug: "rustaceans".to_string(),
            name: "Rustaceans".to_string(),
            is_public: true,
        };
        ready(create(State(use_case), Extension(identity), Json(request)))
    }

    fn respond(outcome: fn() -> Failure<CreateCommunityError>) -> Response {
        call(Arc::new(StubCreateCommunityUseCase(outcome)))
    }

    fn assert_body(response: Response, status: StatusCode, code: &str, message: &str) {
        assert_eq!(response.status(), status);
        assert_eq!(
            body_of(response),
            format!(r#"{{"code":"{code}","message":"{message}"}}"#)
        );
    }

    #[test]
    fn the_identity_subject_is_the_owner_of_the_command() {
        let spy = Arc::new(SpyCreateCommunityUseCase::default());

        let response = call(spy.clone());

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            *spy.owner_ids
                .lock()
                .expect("invariante: mutex sin envenenar"),
            [OWNER]
        );
    }

    #[test]
    fn a_fault_returns_500_with_the_d7_body() {
        let response = respond(|| Failure::Unexpected(Fault::new("db.internal:5432 refused")));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_validation_field_returns_500() {
        let response = respond(|| Failure::expected(Validation { field: "topic" }));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn an_unmapped_conflict_resource_returns_500() {
        let response = respond(|| Failure::expected(Conflict { resource: "name" }));

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body_of(response), D7_BODY);
    }

    #[test]
    fn a_slug_validation_returns_400_communities_invalid_community_slug() {
        assert_body(
            respond(|| Failure::expected(Validation { field: "slug" })),
            StatusCode::BAD_REQUEST,
            "COMMUNITIES_INVALID_COMMUNITY_SLUG",
            "Please enter a valid community slug.",
        );
    }

    #[test]
    fn a_name_validation_returns_400_communities_invalid_community_name() {
        assert_body(
            respond(|| Failure::expected(Validation { field: "name" })),
            StatusCode::BAD_REQUEST,
            "COMMUNITIES_INVALID_COMMUNITY_NAME",
            "Please enter a valid community name.",
        );
    }

    #[test]
    fn a_slug_conflict_returns_409_communities_slug_already_exists() {
        assert_body(
            respond(|| Failure::expected(Conflict { resource: "slug" })),
            StatusCode::CONFLICT,
            "COMMUNITIES_SLUG_ALREADY_EXISTS",
            "This community slug is already in use.",
        );
    }

    #[test]
    fn a_nil_owner_returns_400_iam_invalid_account_id() {
        assert_body(
            respond(|| Failure::expected(InvalidOwner::Nil)),
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_ACCOUNT_ID",
            "The account ID is invalid.",
        );
    }

    #[test]
    fn a_malformed_owner_returns_400_iam_invalid_account_id_format() {
        assert_body(
            respond(|| Failure::expected(InvalidOwner::WrongFormat)),
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_ACCOUNT_ID_FORMAT",
            "The account ID format is invalid.",
        );
    }
}

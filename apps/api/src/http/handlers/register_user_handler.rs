use crate::http::dto::errors::api_error_response::ApiErrorResponse;
use crate::http::dto::requests::register_user_request::RegisterUserRequest;
use crate::http::dto::responses::register_user_response::RegisterUserResponse;
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use iam::application::dto::input::register_user_dto::RegisterUserDto;
use iam::application::errors::application_error::ApplicationError;
use shared::application::common_application_error::CommonApplicationError;

pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>,
) -> Response {
    match state.register_user.execute(RegisterUserDto::from(request)) {
        Ok(user) => (StatusCode::CREATED, Json(RegisterUserResponse::from(user))).into_response(),
        Err(err) => map_application_error(err),
    }
}

pub fn map_application_error(error: ApplicationError) -> Response {
    match error {
        ApplicationError::UsernameAlreadyExists => {
            let body = ApiErrorResponse {
                code: "USERNAME_ALREADY_EXISTS".to_string(),
                message: "Username is already in use".to_string(),
            };
            (StatusCode::CONFLICT, Json(body)).into_response()
        }
        ApplicationError::EmailAlreadyExists => {
            let body = ApiErrorResponse {
                code: "EMAIL_ALREADY_EXISTS".to_string(),
                message: "E-mail is already in use".to_string(),
            };
            (StatusCode::CONFLICT, Json(body)).into_response()
        }
        ApplicationError::InvalidUsername => {
            let body = ApiErrorResponse {
                code: "INVALID_USERNAME".to_string(),
                message: "Invalid username".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidEmail => {
            let body = ApiErrorResponse {
                code: "INVALID_EMAIL".to_string(),
                message: "Invalid e-mail".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::InvalidPassword => {
            let body = ApiErrorResponse {
                code: "INVALID_PASSWORD".to_string(),
                message: "Invalid password".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
        ApplicationError::Common(CommonApplicationError::Infrastructure) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        ApplicationError::Common(CommonApplicationError::Unauthorized) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

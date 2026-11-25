//! Characterization of the public `/auth/*` HTTP contract as it is on `main`.
//!
//! Tests marked "CURRENT BEHAVIOR" pin something that looks unintended. They
//! are here so the refactor does not change it by accident; changing them is
//! a conscious behavior change, not a refactor.

mod common;

use common::*;
use http::{Request, StatusCode, header};
use serde_json::json;

// ---------- /auth/register ----------

#[tokio::test]
async fn register_returns_201_with_uuid_username_and_email() {
    let app = app().await;

    let response = register(&app, "ana", "ana@example.com", PASSWORD).await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "{}",
        response.raw_body
    );
    let body = response.body.as_object().unwrap();
    assert_eq!(body.len(), 3, "unexpected fields: {:?}", body.keys());
    assert_eq!(body["username"], "ana");
    assert_eq!(body["email"], "ana@example.com");
    let uuid = body["uuid"].as_str().unwrap();
    assert_eq!(uuid.len(), 36, "uuid: {uuid}");
    assert_eq!(uuid.matches('-').count(), 4, "uuid: {uuid}");
}

#[tokio::test]
async fn register_normalizes_email_to_lowercase_and_trims_username() {
    let app = app().await;

    let response = register(&app, "  ana  ", "  Ana@Example.COM ", PASSWORD).await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "{}",
        response.raw_body
    );
    assert_eq!(response.body["username"], "ana");
    assert_eq!(response.body["email"], "ana@example.com");
}

#[tokio::test]
async fn register_generates_a_different_uuid_per_account() {
    let app = app().await;

    let first = register(&app, "ana", "ana@example.com", PASSWORD).await;
    let second = register(&app, "bob", "bob@example.com", PASSWORD).await;

    assert_ne!(first.body["uuid"], second.body["uuid"]);
}

#[tokio::test]
async fn register_rejects_duplicated_email_with_409() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, "other", "ana@example.com", PASSWORD).await;

    assert_error(
        &response,
        StatusCode::CONFLICT,
        "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS",
        "This email is already in use.",
    );
}

#[tokio::test]
async fn register_rejects_duplicated_username_with_409() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, "ana", "other@example.com", PASSWORD).await;

    assert_error(
        &response,
        StatusCode::CONFLICT,
        "IAM_ACCOUNT_USERNAME_ALREADY_EXISTS",
        "This username is already in use.",
    );
}

#[tokio::test]
async fn register_checks_email_before_username_when_both_are_taken() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, "ana", "ana@example.com", PASSWORD).await;

    assert_eq!(response.body["code"], "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS");
}

/// CURRENT BEHAVIOR: the duplicate check compares the raw input against the
/// stored (lowercased) email, so a different casing registers a second
/// account with the same email.
#[tokio::test]
async fn register_accepts_same_email_with_different_casing() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, "other", "ANA@example.com", PASSWORD).await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "{}",
        response.raw_body
    );
    assert_eq!(response.body["email"], "ana@example.com");
}

/// CURRENT BEHAVIOR: same as above for usernames with surrounding spaces.
#[tokio::test]
async fn register_accepts_same_username_with_surrounding_spaces() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, " ana ", "other@example.com", PASSWORD).await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "{}",
        response.raw_body
    );
    assert_eq!(response.body["username"], "ana");
}

/// CURRENT BEHAVIOR: the conflict is checked with the raw input before the
/// username format is validated, so an invalid username with a taken email
/// answers 409 instead of 400.
#[tokio::test]
async fn register_with_invalid_username_and_taken_email_returns_409() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = register(&app, "ab", "ana@example.com", PASSWORD).await;

    assert_error(
        &response,
        StatusCode::CONFLICT,
        "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS",
        "This email is already in use.",
    );
}

#[tokio::test]
async fn register_rejects_password_shorter_than_8_chars() {
    let app = app().await;

    let response = register(&app, "ana", "ana@example.com", "1234567").await;

    assert_error(
        &response,
        StatusCode::BAD_REQUEST,
        "IAM_PASSWORD_TOO_SHORT",
        "Password must be at least 8 characters long.",
    );
}

#[tokio::test]
async fn register_accepts_password_of_exactly_8_chars() {
    let app = app().await;

    let response = register(&app, "ana", "ana@example.com", "12345678").await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "{}",
        response.raw_body
    );
}

#[tokio::test]
async fn register_validates_password_before_username_and_email() {
    let app = app().await;

    let response = register(&app, "x", "not-an-email", "short").await;

    assert_eq!(response.body["code"], "IAM_PASSWORD_TOO_SHORT");
}

#[tokio::test]
async fn register_rejects_invalid_usernames() {
    let app = app().await;

    for username in ["", "   ", "ab", "a b c", &"a".repeat(33)] {
        let response = register(&app, username, "ana@example.com", PASSWORD).await;

        assert_error(
            &response,
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_USERNAME",
            "Please enter a valid username.",
        );
    }
}

#[tokio::test]
async fn register_accepts_username_length_bounds() {
    let app = app().await;

    let short = register(&app, "abc", "a@example.com", PASSWORD).await;
    let long = register(&app, &"b".repeat(32), "b@example.com", PASSWORD).await;

    assert_eq!(short.status, StatusCode::CREATED, "{}", short.raw_body);
    assert_eq!(long.status, StatusCode::CREATED, "{}", long.raw_body);
}

#[tokio::test]
async fn register_rejects_invalid_emails() {
    let app = app().await;

    for email in [
        "",
        "plain",
        "a@b",
        "@example.com",
        "ana@",
        "a@b@example.com",
    ] {
        let response = register(&app, "ana", email, PASSWORD).await;

        assert_error(
            &response,
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_EMAIL",
            "Please enter a valid email address.",
        );
    }
}

#[tokio::test]
async fn register_with_missing_field_returns_422() {
    let app = app().await;

    let response = send(
        &app,
        post_json(
            "/auth/register",
            json!({ "username": "ana", "password": PASSWORD }),
        ),
    )
    .await;

    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn register_with_malformed_json_returns_400() {
    let app = app().await;
    let request = Request::post("/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body("{not json".into())
        .unwrap();

    let response = send(&app, request).await;

    assert_eq!(response.status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_without_json_content_type_returns_415() {
    let app = app().await;
    let request = Request::post("/auth/register")
        .body(
            json!({ "username": "ana", "email": "ana@example.com", "password": PASSWORD })
                .to_string()
                .into(),
        )
        .unwrap();

    let response = send(&app, request).await;

    assert_eq!(response.status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

// ---------- /auth/authenticate ----------

async fn authenticate(app: &axum::Router, username: &str, password: &str) -> TestResponse {
    send(
        app,
        post_json(
            "/auth/authenticate",
            json!({ "username": username, "password": password }),
        ),
    )
    .await
}

#[tokio::test]
async fn authenticate_unknown_username_returns_401_login_failed() {
    let app = app().await;

    let response = authenticate(&app, "ghost", PASSWORD).await;

    assert_error(
        &response,
        StatusCode::UNAUTHORIZED,
        "IAM_LOGIN_FAILED",
        "The username or password is incorrect.",
    );
}

#[tokio::test]
async fn authenticate_with_wrong_password_returns_401_login_failed() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = authenticate(&app, "ana", "wrong-password").await;

    assert_error(
        &response,
        StatusCode::UNAUTHORIZED,
        "IAM_LOGIN_FAILED",
        "The username or password is incorrect.",
    );
}

#[tokio::test]
async fn authenticate_unverified_account_returns_401_cannot_authenticate() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = authenticate(&app, "ana", PASSWORD).await;

    assert_error(
        &response,
        StatusCode::UNAUTHORIZED,
        "IAM_CANNOT_AUTHENTICATE",
        "This account is not allowed to sign in at the moment.",
    );
}

/// Authentication looks the account up by username only.
#[tokio::test]
async fn authenticate_by_email_returns_401() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = authenticate(&app, "ana@example.com", PASSWORD).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.body["code"], "IAM_LOGIN_FAILED");
}

// ---------- /auth/verify ----------

async fn verify(app: &axum::Router, email: &str, code: u32) -> TestResponse {
    send(
        app,
        post_json(
            "/auth/verify",
            json!({ "email": email, "validation_code": code }),
        ),
    )
    .await
}

#[tokio::test]
async fn verify_unknown_email_returns_404() {
    let app = app().await;

    let response = verify(&app, "ghost@example.com", 123_456).await;

    assert_error(
        &response,
        StatusCode::NOT_FOUND,
        "IAM_ACCOUNT_NOT_FOUND",
        "No account was found with the provided information.",
    );
}

#[tokio::test]
async fn verify_with_code_out_of_6_digit_range_returns_400() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    for code in [0, 99_999, 1_000_000] {
        let response = verify(&app, "ana@example.com", code).await;

        assert_error(
            &response,
            StatusCode::BAD_REQUEST,
            "IAM_INVALID_CODE_VALIDATION",
            "The verification code is invalid.",
        );
    }
}

/// The real code is random and not exposed by the API. Two different codes
/// cannot both be the right one, so at least one must be rejected; the
/// account stays unverified unless one of them happened to match.
#[tokio::test]
async fn verify_with_wrong_code_returns_400_invalid_verification() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let first = verify(&app, "ana@example.com", 100_000).await;
    let second = verify(&app, "ana@example.com", 999_999).await;

    let rejected = if first.status == StatusCode::OK {
        second
    } else {
        first
    };
    assert_error(
        &rejected,
        StatusCode::BAD_REQUEST,
        "IAM_ACCOUNT_INVALID_VERIFICATION",
        "Verification process is invalid.",
    );
}

#[tokio::test]
async fn verify_with_negative_or_string_code_returns_422() {
    let app = app().await;

    for code in [json!(-1), json!("123456")] {
        let response = send(
            &app,
            post_json(
                "/auth/verify",
                json!({ "email": "ana@example.com", "validation_code": code }),
            ),
        )
        .await;

        assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY, "{code}");
    }
}

// ---------- /auth/identify ----------

async fn identify(app: &axum::Router, identify: &str) -> TestResponse {
    send(
        app,
        post_json("/auth/identify", json!({ "identify": identify })),
    )
    .await
}

#[tokio::test]
async fn identify_by_username_returns_username_and_status() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = identify(&app, "ana").await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.raw_body);
    assert_eq!(
        response.body,
        json!({ "username": "ana", "status": "registered" })
    );
}

#[tokio::test]
async fn identify_by_email_returns_username_and_status() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = identify(&app, "ana@example.com").await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.raw_body);
    assert_eq!(
        response.body,
        json!({ "username": "ana", "status": "registered" })
    );
}

#[tokio::test]
async fn identify_unknown_returns_404() {
    let app = app().await;

    let response = identify(&app, "ghost").await;

    assert_error(
        &response,
        StatusCode::NOT_FOUND,
        "IAM_ACCOUNT_NOT_FOUND",
        "No account was found with the provided information.",
    );
}

/// CURRENT BEHAVIOR: lookup is exact; the input is not normalized.
#[tokio::test]
async fn identify_with_different_email_casing_returns_404() {
    let app = app().await;
    register(&app, "ana", "ana@example.com", PASSWORD).await;

    let response = identify(&app, "ANA@example.com").await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn auth_routes_do_not_require_a_token() {
    let app = app().await;

    let response = identify(&app, "ghost").await;

    assert_ne!(response.status, StatusCode::UNAUTHORIZED);
}

// ---------- routing ----------

/// CURRENT BEHAVIOR: the auth middleware of the protected router also wraps
/// the fallback, so an unknown route answers 401 to anonymous callers.
#[tokio::test]
async fn unknown_route_without_token_returns_401() {
    let app = app().await;

    let response = send(&app, post_json("/auth/sign-up", json!({}))).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.raw_body, "");
}

#[tokio::test]
async fn unknown_route_with_valid_token_returns_404() {
    let app = app().await;
    let auth = format!("Bearer {}", valid_token_for(ACCOUNT_ID));

    let response = send(&app, post_json_with_auth("/auth/sign-up", &auth, json!({}))).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn auth_routes_only_accept_post() {
    let app = app().await;

    for path in [
        "/auth/register",
        "/auth/authenticate",
        "/auth/verify",
        "/auth/identify",
    ] {
        let response = send(&app, Request::get(path).body("".into()).unwrap()).await;

        assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED, "{path}");
    }
}

#[tokio::test]
async fn each_app_instance_has_its_own_in_memory_state() {
    let first = app().await;
    register(&first, "ana", "ana@example.com", PASSWORD).await;

    let second = app().await;
    let response = register(&second, "ana", "ana@example.com", PASSWORD).await;

    assert_eq!(response.status, StatusCode::CREATED);
}

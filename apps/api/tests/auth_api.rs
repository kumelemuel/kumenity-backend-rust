//! Characterization tests for the `/auth` HTTP API.
//!
//! They pin the behavior observed on `main` (status codes, JSON shapes and
//! error codes) so it can be refactored safely. If one of them fails after a
//! refactor, the observable contract changed: decide whether that was intended
//! before touching the test.

mod common;

use common::*;
use http::StatusCode;
use serde_json::json;

// ---------- POST /auth/sign-up ----------

#[tokio::test]
async fn sign_up_returns_201_with_uuid_username_and_email() {
    let app = app();

    let res = sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    assert_eq!(res.status, StatusCode::CREATED);
    assert_eq!(res.body["username"], "alice");
    assert_eq!(res.body["email"], "alice@example.com");
    let uuid = res.body["uuid"].as_str().expect("uuid must be a string");
    assert_eq!(uuid.len(), 36, "uuid must be a hyphenated UUID: {uuid}");
    assert_eq!(
        res.body.as_object().unwrap().len(),
        3,
        "unexpected fields: {}",
        res.body
    );
}

#[tokio::test]
async fn sign_up_with_taken_email_returns_409() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = sign_up(&app, "alice2", "alice@example.com", "supersecret").await;

    assert_eq!(res.status, StatusCode::CONFLICT);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS");
    assert_eq!(res.body["message"], "This email is already in use.");
}

#[tokio::test]
async fn sign_up_with_taken_username_returns_409() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = sign_up(&app, "alice", "other@example.com", "supersecret").await;

    assert_eq!(res.status, StatusCode::CONFLICT);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_USERNAME_ALREADY_EXISTS");
    assert_eq!(res.body["message"], "This username is already in use.");
}

#[tokio::test]
async fn sign_up_with_short_password_returns_400() {
    let res = sign_up(&app(), "alice", "alice@example.com", "short").await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_PASSWORD_TOO_SHORT");
    assert_eq!(
        res.body["message"],
        "Password must be at least 8 characters long."
    );
}

#[tokio::test]
async fn sign_up_with_invalid_username_returns_400() {
    let res = sign_up(&app(), "al", "alice@example.com", "supersecret").await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_INVALID_USERNAME");
}

#[tokio::test]
async fn sign_up_with_invalid_email_returns_400() {
    let res = sign_up(&app(), "alice", "not-an-email", "supersecret").await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_INVALID_EMAIL");
}

#[tokio::test]
async fn sign_up_checks_password_before_username_and_email_format() {
    let res = sign_up(&app(), "al", "not-an-email", "short").await;

    assert_eq!(res.body["code"], "IAM_PASSWORD_TOO_SHORT");
}

#[tokio::test]
async fn sign_up_checks_uniqueness_before_password_policy() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = sign_up(&app, "bob", "alice@example.com", "short").await;

    assert_eq!(res.body["code"], "IAM_ACCOUNT_EMAIL_ALREADY_EXISTS");
}

#[tokio::test]
async fn sign_up_with_missing_fields_is_rejected_by_the_json_extractor() {
    let res = send(
        &app(),
        post_json("/auth/sign-up", json!({ "username": "alice" })),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn sign_up_without_json_content_type_returns_415() {
    let request = http::Request::post("/auth/sign-up")
        .body(axum::body::Body::from(
            json!({ "username": "alice", "email": "alice@example.com", "password": "supersecret" })
                .to_string(),
        ))
        .unwrap();

    let res = send(&app(), request).await;

    assert_eq!(res.status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

// ---------- POST /auth/identify ----------

#[tokio::test]
async fn identify_by_email_returns_username_and_registered_status() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = send(
        &app,
        post_json("/auth/identify", json!({ "identify": "alice@example.com" })),
    )
    .await;

    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(
        res.body,
        json!({ "username": "alice", "status": "registered" })
    );
}

#[tokio::test]
async fn identify_by_username_returns_username_and_status() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = send(
        &app,
        post_json("/auth/identify", json!({ "identify": "alice" })),
    )
    .await;

    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(
        res.body,
        json!({ "username": "alice", "status": "registered" })
    );
}

#[tokio::test]
async fn identify_unknown_account_returns_404() {
    let res = send(
        &app(),
        post_json("/auth/identify", json!({ "identify": "nobody" })),
    )
    .await;

    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_NOT_FOUND");
    assert_eq!(
        res.body["message"],
        "No account was found with the provided information."
    );
}

// ---------- POST /auth/verify ----------

#[tokio::test]
async fn verify_unknown_email_returns_404() {
    let res = send(
        &app(),
        post_json(
            "/auth/verify",
            json!({ "email": "nobody@example.com", "validation_code": 123456 }),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_NOT_FOUND");
}

#[tokio::test]
async fn verify_with_out_of_range_code_returns_400() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = send(
        &app,
        post_json(
            "/auth/verify",
            json!({ "email": "alice@example.com", "validation_code": 42 }),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_INVALID_CODE_VALIDATION");
}

#[tokio::test]
async fn verify_with_wrong_code_returns_400_and_account_stays_registered() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    // The real code is random (100000..=999999) and is not exposed by the API.
    // Trying two distinct codes guarantees at least one of them is wrong.
    let mut wrong = None;
    for code in [100_000, 100_001] {
        let res = send(
            &app,
            post_json(
                "/auth/verify",
                json!({ "email": "alice@example.com", "validation_code": code }),
            ),
        )
        .await;
        if res.status != StatusCode::OK {
            wrong = Some(res);
            break;
        }
    }
    let res = wrong.expect("at least one code must be wrong");

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_INVALID_VERIFICATION");
    assert_eq!(res.body["message"], "Verification process is invalid.");
}

// ---------- POST /auth/sign-in ----------

#[tokio::test]
async fn sign_in_unknown_username_returns_404() {
    let res = send(
        &app(),
        post_json(
            "/auth/sign-in",
            json!({ "username": "nobody", "password": "supersecret" }),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert_eq!(res.body["code"], "IAM_ACCOUNT_NOT_FOUND");
}

#[tokio::test]
async fn sign_in_with_wrong_password_returns_401() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = send(
        &app,
        post_json(
            "/auth/sign-in",
            json!({ "username": "alice", "password": "wrongpassword" }),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
    assert_eq!(res.body["code"], "IAM_LOGIN_FAILED");
    assert_eq!(
        res.body["message"],
        "The username or password is incorrect."
    );
}

#[tokio::test]
async fn sign_in_unverified_account_with_right_password_returns_401() {
    let app = app();
    sign_up(&app, "alice", "alice@example.com", "supersecret").await;

    let res = send(
        &app,
        post_json(
            "/auth/sign-in",
            json!({ "username": "alice", "password": "supersecret" }),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
    assert_eq!(res.body["code"], "IAM_CANNOT_AUTHENTICATE");
}

// ---------- routing ----------

#[tokio::test]
async fn unknown_auth_route_returns_404() {
    let res = send(&app(), post_json("/auth/unknown", json!({}))).await;

    assert_eq!(res.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn auth_routes_only_accept_post() {
    let request = http::Request::get("/auth/sign-up")
        .body(axum::body::Body::empty())
        .unwrap();

    let res = send(&app(), request).await;

    assert_eq!(res.status, StatusCode::METHOD_NOT_ALLOWED);
}

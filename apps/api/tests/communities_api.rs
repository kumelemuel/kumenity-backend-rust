//! Characterization tests for the `/communities` HTTP API, including the
//! bearer-token authentication it relies on.
//!
//! They pin the behavior observed on `main`. If one of them fails after a
//! refactor, the observable contract changed: decide whether that was intended
//! before touching the test.

mod common;

use common::*;
use http::StatusCode;
use serde_json::{Value, json};

const CREATE: &str = "/communities/create";

fn community(slug: &str, name: &str, is_public: bool) -> Value {
    json!({ "slug": slug, "name": name, "is_public": is_public })
}

fn bearer(token: &str) -> String {
    format!("Bearer {token}")
}

// ---------- authentication ----------

#[tokio::test]
async fn create_without_authorization_header_returns_401_with_empty_body() {
    let res = send(
        &app(),
        post_json(CREATE, community("rust-lovers", "Rust Lovers", true)),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
    assert_eq!(res.raw_body, "");
}

#[tokio::test]
async fn create_with_non_bearer_scheme_returns_401() {
    let token = valid_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &format!("Basic {token}"),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_bearer_scheme_in_lowercase_returns_401() {
    let token = valid_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &format!("bearer {token}"),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_bearer_but_no_token_returns_401() {
    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            "Bearer",
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_garbage_token_returns_401() {
    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            "Bearer not-a-jwt",
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_token_signed_by_another_secret_returns_401() {
    let token = token_for(ACCOUNT_ID, "another-secret", u64::MAX / 2);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_expired_token_returns_401() {
    let token = expired_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_with_token_whose_subject_is_not_a_uuid_returns_400() {
    let token = valid_token_for("not-a-uuid");

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "IAM_INVALID_ACCOUNT_ID_FORMAT");
}

#[tokio::test]
async fn malformed_body_is_rejected_before_authentication() {
    let res = send(&app(), post_json(CREATE, json!({ "slug": "rust-lovers" }))).await;

    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
}

// ---------- creation ----------

#[tokio::test]
async fn create_returns_200_with_id_name_and_slug() {
    let token = valid_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(res.body["slug"], "rust-lovers");
    assert_eq!(res.body["name"], "Rust Lovers");
    let id = res.body["id"].as_str().expect("id must be a string");
    assert_eq!(id.len(), 36, "id must be a hyphenated UUID: {id}");
    assert_eq!(
        res.body.as_object().unwrap().len(),
        3,
        "unexpected fields: {}",
        res.body
    );
}

#[tokio::test]
async fn create_private_community_returns_200() {
    let token = valid_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("secret-club", "Secret Club", false),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::OK);
}

#[tokio::test]
async fn create_trims_name_and_slug() {
    let token = valid_token_for(ACCOUNT_ID);

    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&token),
            community("  rust-lovers  ", "  Rust Lovers  ", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(res.body["slug"], "rust-lovers");
    assert_eq!(res.body["name"], "Rust Lovers");
}

#[tokio::test]
async fn create_with_existing_slug_returns_409_even_for_another_account() {
    let app = app();
    send(
        &app,
        post_json_with_auth(
            CREATE,
            &bearer(&valid_token_for(ACCOUNT_ID)),
            community("rust-lovers", "Rust Lovers", true),
        ),
    )
    .await;

    let other_account = "0b8e2f4c-5d6a-4b3c-8e9f-1a2b3c4d5e6f";
    let res = send(
        &app,
        post_json_with_auth(
            CREATE,
            &bearer(&valid_token_for(other_account)),
            community("rust-lovers", "Other Name", false),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::CONFLICT);
    assert_eq!(res.body["code"], "COMMUNITIES_SLUG_ALREADY_EXISTS");
}

#[tokio::test]
async fn create_with_invalid_slug_returns_400() {
    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&valid_token_for(ACCOUNT_ID)),
            community("rust lovers", "Rust Lovers", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "COMMUNITIES_INVALID_COMMUNITY_SLUG");
}

#[tokio::test]
async fn create_with_invalid_name_returns_400() {
    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&valid_token_for(ACCOUNT_ID)),
            community("rust-lovers", "Rust", true),
        ),
    )
    .await;

    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.body["code"], "COMMUNITIES_INVALID_COMMUNITY_NAME");
}

#[tokio::test]
async fn create_validates_slug_before_account_id_and_name() {
    let res = send(
        &app(),
        post_json_with_auth(
            CREATE,
            &bearer(&valid_token_for("not-a-uuid")),
            community("x", "x", true),
        ),
    )
    .await;

    assert_eq!(res.body["code"], "COMMUNITIES_INVALID_COMMUNITY_SLUG");
}

// ---------- routing ----------

#[tokio::test]
async fn there_is_no_public_listing_route_yet() {
    let request = http::Request::get("/communities")
        .body(axum::body::Body::empty())
        .unwrap();

    let res = send(&app(), request).await;

    assert_eq!(res.status, StatusCode::NOT_FOUND);
}

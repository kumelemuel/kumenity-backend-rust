//! Characterization of the protected `/communities/*` HTTP contract and of
//! the bearer-token middleware, as they are on `main`.
//!
//! Tokens are minted here with the same secret and claim format the server
//! issues (HS256, `sub` = account id, `exp`), because the API offers no way
//! to verify an account and obtain a real token.

mod common;

use axum::Router;
use common::*;
use http::{Request, StatusCode, header};
use serde_json::{Value, json};

const PATH: &str = "/communities/create";

fn community(slug: &str, name: &str) -> Value {
    json!({ "slug": slug, "name": name, "is_public": true })
}

async fn create_as(app: &Router, sub: &str, body: Value) -> TestResponse {
    let auth = format!("Bearer {}", valid_token_for(sub));
    send(app, post_json_with_auth(PATH, &auth, body)).await
}

async fn create(app: &Router, body: Value) -> TestResponse {
    create_as(app, ACCOUNT_ID, body).await
}

// ---------- auth middleware ----------

async fn assert_rejected_with(authorization: Option<&str>) {
    let app = app().await;
    let body = community("rustaceans", "Rustaceans");
    let request = match authorization {
        Some(value) => post_json_with_auth(PATH, value, body),
        None => post_json(PATH, body),
    };

    let response = send(&app, request).await;

    assert_eq!(
        response.status,
        StatusCode::UNAUTHORIZED,
        "{authorization:?}"
    );
    assert_eq!(response.raw_body, "", "401 has an empty body");
}

#[tokio::test]
async fn create_without_authorization_header_returns_401() {
    assert_rejected_with(None).await;
}

#[tokio::test]
async fn create_with_non_bearer_scheme_returns_401() {
    let token = valid_token_for(ACCOUNT_ID);
    assert_rejected_with(Some(&format!("Basic {token}"))).await;
    assert_rejected_with(Some(&format!("bearer {token}"))).await;
    assert_rejected_with(Some(&token)).await;
}

#[tokio::test]
async fn create_with_garbage_token_returns_401() {
    assert_rejected_with(Some("Bearer not-a-jwt")).await;
    assert_rejected_with(Some("Bearer ")).await;
}

#[tokio::test]
async fn create_with_expired_token_returns_401() {
    assert_rejected_with(Some(&format!("Bearer {}", expired_token_for(ACCOUNT_ID)))).await;
}

#[tokio::test]
async fn create_with_token_signed_by_another_secret_returns_401() {
    let token = token_for(ACCOUNT_ID, "another-secret", u64::MAX / 2);
    assert_rejected_with(Some(&format!("Bearer {token}"))).await;
}

#[tokio::test]
async fn middleware_runs_before_body_extraction() {
    let app = app().await;
    let request = Request::post(PATH).body("{not json".into()).unwrap();

    let response = send(&app, request).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
}

// ---------- /communities/create ----------

#[tokio::test]
async fn create_returns_200_with_id_name_and_slug() {
    let app = app().await;

    let response = create(&app, community("rustaceans", "Rustaceans")).await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.raw_body);
    let body = response.body.as_object().unwrap();
    assert_eq!(body.len(), 3, "unexpected fields: {:?}", body.keys());
    assert_eq!(body["name"], "Rustaceans");
    assert_eq!(body["slug"], "rustaceans");
    let id = body["id"].as_str().unwrap();
    assert_eq!(id.len(), 36, "id: {id}");
    assert_eq!(id.matches('-').count(), 4, "id: {id}");
}

#[tokio::test]
async fn create_normalizes_slug_to_lowercase_and_trims_name_and_slug() {
    let app = app().await;

    let response = create(&app, community("  Rust-Lang ", "  Rust Lovers  ")).await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.raw_body);
    assert_eq!(response.body["slug"], "rust-lang");
    assert_eq!(response.body["name"], "Rust Lovers");
}

#[tokio::test]
async fn create_accepts_private_communities() {
    let app = app().await;

    let response = create(
        &app,
        json!({ "slug": "secret", "name": "Secret club", "is_public": false }),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.raw_body);
}

#[tokio::test]
async fn create_rejects_duplicated_slug_with_409() {
    let app = app().await;
    create(&app, community("rustaceans", "Rustaceans")).await;

    let response = create(&app, community("rustaceans", "Other name")).await;

    assert_error(
        &response,
        StatusCode::CONFLICT,
        "COMMUNITIES_SLUG_ALREADY_EXISTS",
        "This community slug is already in use.",
    );
}

#[tokio::test]
async fn duplicated_slug_is_detected_case_insensitively() {
    let app = app().await;
    create(&app, community("rustaceans", "Rustaceans")).await;

    let response = create(&app, community("RUSTACEANS", "Other name")).await;

    assert_eq!(response.status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn duplicated_slug_is_rejected_for_any_account() {
    let app = app().await;
    create(&app, community("rustaceans", "Rustaceans")).await;

    let response = create_as(
        &app,
        "0b7e7c1e-1f0e-4a43-9c55-3a8f4f1f2d22",
        community("rustaceans", "Rustaceans"),
    )
    .await;

    assert_eq!(response.status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_rejects_invalid_slugs() {
    let app = app().await;

    for slug in [
        "",
        "ab",
        "with space",
        "rust_lang",
        "rust1",
        "-rust",
        "rust-",
        "rust--lang",
        "ñandú",
        &"a".repeat(33),
    ] {
        let response = create(&app, community(slug, "Valid name")).await;

        assert_error(
            &response,
            StatusCode::BAD_REQUEST,
            "COMMUNITIES_INVALID_COMMUNITY_SLUG",
            "Please enter a valid community slug.",
        );
    }
}

#[tokio::test]
async fn create_rejects_invalid_names() {
    let app = app().await;

    for name in ["", "    ", "abcd", &"a".repeat(65)] {
        let response = create(&app, community("rustaceans", name)).await;

        assert_error(
            &response,
            StatusCode::BAD_REQUEST,
            "COMMUNITIES_INVALID_COMMUNITY_NAME",
            "Please enter a valid community name.",
        );
    }
}

#[tokio::test]
async fn create_accepts_slug_and_name_length_bounds() {
    let app = app().await;

    let min = create(&app, community("abc", "abcde")).await;
    let max = create(&app, community(&"b".repeat(32), &"n".repeat(64))).await;

    assert_eq!(min.status, StatusCode::OK, "{}", min.raw_body);
    assert_eq!(max.status, StatusCode::OK, "{}", max.raw_body);
}

#[tokio::test]
async fn create_validates_slug_before_name() {
    let app = app().await;

    let response = create(&app, community("!", "")).await;

    assert_eq!(response.body["code"], "COMMUNITIES_INVALID_COMMUNITY_SLUG");
}

/// The token is valid but its subject is not a UUID.
#[tokio::test]
async fn create_with_non_uuid_subject_returns_400() {
    let app = app().await;

    let response = create_as(&app, "not-a-uuid", community("rustaceans", "Rustaceans")).await;

    assert_error(
        &response,
        StatusCode::BAD_REQUEST,
        "IAM_INVALID_ACCOUNT_ID_FORMAT",
        "The account ID format is invalid.",
    );
}

#[tokio::test]
async fn create_with_nil_uuid_subject_returns_400() {
    let app = app().await;

    let response = create_as(
        &app,
        "00000000-0000-0000-0000-000000000000",
        community("rustaceans", "Rustaceans"),
    )
    .await;

    assert_error(
        &response,
        StatusCode::BAD_REQUEST,
        "IAM_INVALID_ACCOUNT_ID",
        "The account ID is invalid.",
    );
}

/// CURRENT BEHAVIOR: the subject is not checked against registered
/// accounts; any well-formed UUID can create communities.
#[tokio::test]
async fn create_does_not_require_the_subject_to_be_a_registered_account() {
    let app = app().await;

    let response = create(&app, community("rustaceans", "Rustaceans")).await;

    assert_eq!(response.status, StatusCode::OK);
}

#[tokio::test]
async fn create_checks_slug_uniqueness_before_subject_format() {
    let app = app().await;
    create(&app, community("rustaceans", "Rustaceans")).await;

    let response = create_as(&app, "not-a-uuid", community("rustaceans", "Rustaceans")).await;

    assert_eq!(response.body["code"], "COMMUNITIES_SLUG_ALREADY_EXISTS");
}

#[tokio::test]
async fn create_with_missing_field_returns_422() {
    let app = app().await;

    let response = create(&app, json!({ "slug": "rustaceans", "name": "Rustaceans" })).await;

    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_only_accepts_post() {
    let app = app().await;
    let request = Request::get(PATH)
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}", valid_token_for(ACCOUNT_ID)),
        )
        .body("".into())
        .unwrap();

    let response = send(&app, request).await;

    assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED);
}

/// The list-public-communities use case exists but is not routed.
#[tokio::test]
async fn there_is_no_route_to_list_communities() {
    let app = app().await;
    let auth = format!("Bearer {}", valid_token_for(ACCOUNT_ID));

    for path in ["/communities", "/communities/public", "/communities/list"] {
        let request = Request::get(path)
            .header(header::AUTHORIZATION, &auth)
            .body("".into())
            .unwrap();

        let response = send(&app, request).await;

        assert_eq!(response.status, StatusCode::NOT_FOUND, "{path}");
    }
}

//! Black-box helpers for HTTP characterization tests.
//!
//! These tests only depend on `api::build_app` and on the HTTP contract
//! (paths, JSON bodies, status codes, JWT format), never on internal types,
//! so they survive moving modules around.
#![allow(dead_code)]

use api::{build_app, config::jwt::JwtConfig};
use axum::{Router, body::Body};
use http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

pub const SECRET: &str = "characterization-secret";

pub struct TestResponse {
    pub status: StatusCode,
    pub body: Value,
    pub raw_body: String,
}

pub fn app() -> Router {
    build_app(JwtConfig {
        secret: SECRET.to_string(),
        expiration_time: 3600,
    })
}

pub async fn send(app: &Router, request: Request<Body>) -> TestResponse {
    let response = app.clone().oneshot(request).await.expect("request failed");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body read failed")
        .to_bytes();
    let raw_body = String::from_utf8_lossy(&bytes).to_string();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    TestResponse {
        status,
        body,
        raw_body,
    }
}

pub fn post_json(path: &str, body: Value) -> Request<Body> {
    Request::post(path)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

pub fn post_json_with_auth(path: &str, authorization: &str, body: Value) -> Request<Body> {
    Request::post(path)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, authorization)
        .body(Body::from(body.to_string()))
        .unwrap()
}

pub async fn sign_up(app: &Router, username: &str, email: &str, password: &str) -> TestResponse {
    send(
        app,
        post_json(
            "/auth/sign-up",
            json!({ "username": username, "email": email, "password": password }),
        ),
    )
    .await
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Mints an HS256 JWT with `sub` and `exp`, the format the API accepts.
pub fn token_for(sub: &str, secret: &str, exp: u64) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &json!({ "sub": sub, "exp": exp }),
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn valid_token_for(sub: &str) -> String {
    token_for(sub, SECRET, now() + 3600)
}

pub fn expired_token_for(sub: &str) -> String {
    token_for(sub, SECRET, now() - 3600)
}

pub const ACCOUNT_ID: &str = "6f1c1a9e-3a3e-4d7b-9d0c-2f5b0f6a7c11";

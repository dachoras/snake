//! Black-box integration tests for the snake backend's auth and redirect endpoints.

mod common;

use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

use common::{build_test_app, get, json_post, send, with_connect_info};

#[tokio::test]
async fn verify_pin_no_pin_configured_returns_200_no_cookie() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let (status, body, headers) = send(
        &router,
        with_connect_info(json_post("/api/verify-pin", json!({ "pin": "anything" }))),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert!(headers.get("set-cookie").is_none());
}

#[tokio::test]
async fn verify_pin_correct_pin_returns_200_with_cookie() {
    let (_tmp, _state, router) = build_test_app(Some("1234")).await;
    let (status, body, headers) = send(
        &router,
        with_connect_info(json_post("/api/verify-pin", json!({ "pin": "1234" }))),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    let cookie = headers
        .get("set-cookie")
        .expect("set-cookie")
        .to_str()
        .expect("str");
    assert!(cookie.contains("SNAKE_PIN="));
}

#[tokio::test]
async fn verify_pin_wrong_pin_returns_401() {
    let (_tmp, _state, router) = build_test_app(Some("1234")).await;
    let (status, body, _) = send(
        &router,
        with_connect_info(json_post("/api/verify-pin", json!({ "pin": "9999" }))),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn verify_pin_too_short_returns_400() {
    let (_tmp, _state, router) = build_test_app(Some("1234")).await;
    let (status, body, _) = send(
        &router,
        with_connect_info(json_post("/api/verify-pin", json!({ "pin": "12" }))),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn login_rejects_scheme_relative_redirect() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let resp = router
        .clone()
        .oneshot(get("/login?redirect=//evil.com"))
        .await
        .expect("oneshot");
    assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        resp.headers().get("location").unwrap().to_str().unwrap(),
        "/"
    );
}

#[tokio::test]
async fn login_rejects_percent_encoded_slash_redirect() {
    let (_tmp, _state, router) = build_test_app(None).await;
    // `%252Fevil` -> the URL-decoded value is `/%2Fevil` (a literal
    // percent-encoded slash), which `is_valid_redirect_url` rejects, so
    // the handler falls through to the safe default of `/`.
    let resp = router
        .clone()
        .oneshot(get("/login?redirect=%252Fevil"))
        .await
        .expect("oneshot");
    assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        resp.headers().get("location").unwrap().to_str().unwrap(),
        "/"
    );
}

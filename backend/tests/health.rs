//! Black-box integration tests for the snake backend's `/health` endpoint.

mod common;

use axum::http::StatusCode;

use common::{build_test_app, get, send};

#[tokio::test]
async fn health_returns_200_with_status_ok() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let (status, body, _) = send(&router, get("/health")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert!(body["timestamp"].is_number());
}

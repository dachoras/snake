//! Black-box integration tests for the snake backend's per-IP rate limiter.

mod common;

use axum::http::StatusCode;
use tower::ServiceExt;

use common::{build_test_app, get, with_connect_info};

#[tokio::test]
async fn rate_limit_429_after_100_requests_in_60s_window() {
    let (_tmp, _state, router) = build_test_app(None).await;
    for i in 0..100 {
        let resp = router
            .clone()
            .oneshot(with_connect_info(get("/api/pin-required")))
            .await
            .expect("oneshot");
        assert_ne!(
            resp.status(),
            StatusCode::TOO_MANY_REQUESTS,
            "request {i} unexpectedly throttled"
        );
    }
    let resp = router
        .clone()
        .oneshot(with_connect_info(get("/api/pin-required")))
        .await
        .expect("oneshot");
    assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
}

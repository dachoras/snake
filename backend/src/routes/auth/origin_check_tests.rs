use super::*;
use crate::config::AppConfig;
use crate::services::rate_limit::RateLimiter;
use crate::state::AppStateInner;
use axum::body::Body;
use axum::http::{Method, Request as HttpRequest};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

fn build_state(base_url: &str) -> AppState {
    let mut server = shared_backend::server::ServerConfig::from_env(crate::config::APP_BRAND);
    server.base_url = base_url.to_string();
    let cfg = AppConfig {
        server,
        page_history_cookie_age_days: 1,
        node_env: "test".to_string(),
        version: "test".to_string(),
    };
    let tmp = tempfile::TempDir::new().expect("tempdir");
    Arc::new(AppStateInner {
        config: cfg,
        data_dir: tmp.path().to_path_buf(),
        leaderboard_file: tmp.path().join("leaderboard.json"),
        web_root: tmp.path().join("frontend"),
        active_sessions: RwLock::new(HashSet::new()),
        rate_limiter: RwLock::new(RateLimiter::new()),
        leaderboard_lock: Arc::new(Mutex::new(())),
        metrics: Arc::new(crate::metrics::Metrics::new("test", 0, 0)),
    })
}

fn post_with_origin(origin: Option<&str>) -> Request {
    let mut b = HttpRequest::builder()
        .method(Method::POST)
        .uri("/api/leaderboard");
    if let Some(o) = origin {
        b = b.header(header::ORIGIN, o);
    }
    b.body(Body::empty()).expect("req")
}

fn assert_forbidden(state: &AppState, origin: Option<&str>, allow_null: bool) {
    let resp = assert_origin_allowed(&post_with_origin(origin), state, allow_null)
        .expect_err("should reject");
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[test]
fn missing_origin_rejected() {
    assert_forbidden(&build_state("https://snake.example.com"), None, false);
}

#[test]
fn matching_origin_allowed() {
    let state = build_state("https://snake.example.com");
    assert!(
        assert_origin_allowed(
            &post_with_origin(Some("https://snake.example.com")),
            &state,
            false
        )
        .is_ok()
    );
}

#[test]
fn cross_origin_rejected() {
    assert_forbidden(
        &build_state("https://snake.example.com"),
        Some("https://evil.example.com"),
        false,
    );
}

#[test]
fn null_origin_rejected_by_default() {
    assert_forbidden(
        &build_state("https://snake.example.com"),
        Some("null"),
        false,
    );
}

#[test]
fn null_origin_allowed_when_opt_in() {
    let state = build_state("https://snake.example.com");
    assert!(assert_origin_allowed(&post_with_origin(Some("null")), &state, true).is_ok());
}

#[test]
fn localhost_wildcard_accepts_any_port() {
    let state = build_state("http://localhost:4501");
    for origin in [
        "http://localhost:4501",
        "http://localhost:5173",
        "http://localhost:8080",
    ] {
        assert!(
            assert_origin_allowed(&post_with_origin(Some(origin)), &state, false).is_ok(),
            "expected {origin} to be accepted"
        );
    }
}

#[test]
fn loopback_wildcard_accepts_any_port() {
    let state = build_state("http://127.0.0.1:4501");
    for origin in [
        "http://127.0.0.1:4501",
        "http://127.0.0.1:5173",
        "http://127.0.0.1",
    ] {
        assert!(
            assert_origin_allowed(&post_with_origin(Some(origin)), &state, false).is_ok(),
            "expected {origin} to be accepted"
        );
    }
}

#[test]
fn production_rejects_loopback_origin() {
    let state = build_state("https://snake.example.com");
    let resp = assert_origin_allowed(
        &post_with_origin(Some("http://localhost:4501")),
        &state,
        false,
    )
    .expect_err("reject");
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[test]
fn localhost_does_not_accept_garbage_port() {
    let state = build_state("http://localhost:4501");
    for bad in ["http://localhost:abc", "http://localhost:"] {
        let resp =
            assert_origin_allowed(&post_with_origin(Some(bad)), &state, false).expect_err("reject");
        assert_eq!(resp.status(), StatusCode::FORBIDDEN, "origin: {bad}");
    }
}

#[test]
fn host_header_same_origin_allowed() {
    let state = build_state("http://localhost:4501");
    let mut req = post_with_origin(Some("http://192.168.1.50:4501"));
    req.headers_mut().insert(
        header::HOST,
        axum::http::HeaderValue::from_static("192.168.1.50:4501"),
    );
    assert!(assert_origin_allowed(&req, &state, false).is_ok());
}

#[test]
fn host_header_mismatch_rejected() {
    let state = build_state("http://localhost:4501");
    let mut req = post_with_origin(Some("http://192.168.1.50:4501"));
    req.headers_mut().insert(
        header::HOST,
        axum::http::HeaderValue::from_static("192.168.1.99:4501"),
    );
    let resp = assert_origin_allowed(&req, &state, false).expect_err("reject");
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

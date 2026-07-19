//! Container smoke tests for `snake`.
//!
//! Run against a live container:
//!   SMOKE_PORT=<host-port> cargo test --test container_smoke -- --ignored --nocapture

use reqwest::Client;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const APP_NAME: &str = "snake";
const DEFAULT_PORT: u16 = 4501;

const FAVICON_CANDIDATES: &[&str] = &["/assets/favicon.png", "/favicon.png"];
const MANIFEST_CANDIDATES: &[&str] = &["/assets/manifest.json", "/manifest.json"];
const CONFIG_CANDIDATES: &[&str] = &["/api/config", "/api/auth/config"];
const SERVICE_WORKER_CANDIDATES: &[&str] = &[
    "/service-worker.js",
    "/api/service-worker.js",
    "/assets/service-worker.js",
];

fn port() -> u16 {
    std::env::var("SMOKE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

fn base_url() -> String {
    format!("http://127.0.0.1:{}", port())
}

fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("reqwest client")
}

/// Snake's `sanitize_player_name` truncates names to 15 UTF-8 chars
/// without altering characters. Build an exactly-15-char alphanumeric
/// tag from the system clock + pid.
fn unique_id() -> String {
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut tag = String::with_capacity(15);
    let mut n = ns;
    for _ in 0..15 {
        let idx = (n % chars.len() as u128) as usize;
        tag.push(chars[idx] as char);
        n /= chars.len() as u128;
        if n == 0 {
            n = std::process::id() as u128 + 1;
        }
    }
    tag
}

async fn wait_for_health() {
    let c = client();
    for _ in 0..30 {
        if let Ok(r) = c.get(format!("{}/health", base_url())).send().await {
            if r.status().is_success() {
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    panic!("container at {} never became healthy", base_url());
}

async fn try_paths(c: &Client, paths: &[&str]) -> Option<reqwest::Response> {
    for p in paths {
        if let Ok(r) = c.get(format!("{}{}", base_url(), p)).send().await {
            if r.status().is_success() {
                return Some(r);
            }
        }
    }
    None
}

// ---------- common tests ----------

#[tokio::test]
#[ignore]
async fn health_returns_200() {
    let c = client();
    let r = c.get(format!("{}/health", base_url())).send().await.unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /health");
}

#[tokio::test]
#[ignore]
async fn root_serves_html() {
    let c = client();
    let r = c.get(&base_url()).send().await.unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /");
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(ct.starts_with("text/html"), "expected text/html, got {ct:?}");
}

#[tokio::test]
#[ignore]
async fn favicon_resolves() {
    let c = client();
    let r = try_paths(&c, FAVICON_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no favicon path returned 2xx: {FAVICON_CANDIDATES:?}"));
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.starts_with("image/") || ct.starts_with("application/octet-stream"),
        "expected image/* (or octet-stream), got {ct:?}"
    );
}

#[tokio::test]
#[ignore]
async fn manifest_parses_as_pwa() {
    let c = client();
    let r = try_paths(&c, MANIFEST_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no manifest path returned 2xx: {MANIFEST_CANDIDATES:?}"));
    let v: Value = r.json().await.unwrap();
    assert!(v["name"].is_string(), "manifest.name must be a string, got {v:?}");
    assert!(v["icons"].is_array(), "manifest.icons must be an array");
}

#[tokio::test]
#[ignore]
async fn config_endpoint_has_site_title() {
    let c = client();
    let r = try_paths(&c, CONFIG_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no config path returned 2xx: {CONFIG_CANDIDATES:?}"));
    let v: Value = r.json().await.unwrap();
    let title = v["siteTitle"]
        .as_str()
        .or_else(|| v["site_title"].as_str())
        .unwrap_or("");
    assert!(
        title.eq_ignore_ascii_case(APP_NAME),
        "expected siteTitle == {APP_NAME:?}, got {title:?}"
    );
}

#[tokio::test]
#[ignore]
async fn service_worker_or_frontend_serves() {
    let c = client();
    let r = try_paths(&c, SERVICE_WORKER_CANDIDATES).await;
    assert!(
        r.is_some(),
        "no service-worker path returned 2xx: {SERVICE_WORKER_CANDIDATES:?}"
    );
}

// ---------- per-app tests: snake ----------

#[tokio::test]
#[ignore]
async fn leaderboard_get_returns_array() {
    wait_for_health().await;
    let c = client();
    let r = c
        .get(format!("{}/api/leaderboard", base_url()))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /api/leaderboard");
    let v: Value = r.json().await.unwrap();
    let arr = v
        .as_array()
        .or_else(|| v["entries"].as_array())
        .expect("leaderboard response must be array or {entries: []}");
    for e in arr {
        assert!(e.is_object(), "leaderboard entry must be object");
    }
}

#[tokio::test]
#[ignore]
async fn leaderboard_post_round_trips() {
    wait_for_health().await;
    let c = client();
    let name = unique_id();
    let payload = serde_json::json!({
        "name": name,
        "score": 42,
        "date": "2026-07-19T00:00:00Z",
    });
    // Origin header required by snake's CSRF defense on POSTs.
    let post = c
        .post(format!("{}/api/leaderboard", base_url()))
        .header("Origin", base_url())
        .header("Referer", format!("{}/", base_url()))
        .json(&payload)
        .send()
        .await
        .unwrap();
    assert!(
        post.status().is_success(),
        "POST leaderboard failed: {}",
        post.status()
    );
    let get = c
        .get(format!("{}/api/leaderboard", base_url()))
        .send()
        .await
        .unwrap();
    let body: Value = get.json().await.unwrap();
    let entries = body
        .as_array()
        .or_else(|| body["entries"].as_array())
        .expect("leaderboard response must be array or {entries: []}");
    let found = entries
        .iter()
        .any(|e| e["name"].as_str() == Some(name.as_str()));
    assert!(found, "submitted entry {name:?} not found in leaderboard");
}

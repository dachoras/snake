//! Origin-header CSRF defense middleware. Pin the `Origin` header on
//! state-changing requests to the configured `base_url`. Applied to
//! `/api/leaderboard` only; non-`POST` methods pass through.

use axum::extract::{Request, State};
use axum::http::{Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::state::AppState;

/// Env-var enabling the `Origin: null` bypass for `curl` clients.
/// Defaults to `false`.
pub const ALLOW_NULL_ORIGIN_ENV: &str = "ALLOW_NULL_ORIGIN";

/// Truthy values: `"true"`, `"1"`, `"on"` (case-insensitive).
#[must_use]
pub fn allow_null_origin_from_env() -> bool {
    matches!(
        std::env::var(ALLOW_NULL_ORIGIN_ENV).ok().as_deref(),
        Some("true" | "TRUE" | "True" | "1" | "on" | "ON" | "On")
    )
}

/// Inspect the request's `Origin` header. `allow_null_origin` is
/// caller-supplied so unit tests pin the policy without touching
/// process-global environment.
pub fn assert_origin_allowed(
    req: &Request,
    state: &AppState,
    allow_null_origin: bool,
) -> Result<(), Box<Response>> {
    let Some(origin) = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
    else {
        tracing::warn!(target: "origin_check", "rejected: missing Origin header");
        return Err(Box::new(forbidden_response()));
    };
    if origin.eq_ignore_ascii_case("null") {
        if allow_null_origin {
            return Ok(());
        }
        tracing::warn!(target: "origin_check", origin = %origin, "rejected: null Origin");
        return Err(Box::new(forbidden_response()));
    }
    let base = state.config.server.base_url.as_str();
    let is_same_origin = if let Some(host) = req
        .headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
    {
        let stripped_origin = origin
            .strip_prefix("http://")
            .or_else(|| origin.strip_prefix("https://"))
            .unwrap_or(origin);
        stripped_origin == host
    } else {
        false
    };

    if is_same_origin || origin_matches(origin, base) {
        Ok(())
    } else {
        tracing::warn!(target: "origin_check", origin = %origin, base_url = %base, "rejected: cross-origin");
        Err(Box::new(forbidden_response()))
    }
}

/// Match `origin` against `base`. Pass when exact, or when `base` is
/// on localhost/`127.0.0.1` and the origin is the same scheme+host on
/// any port (developer ergonomics).
fn origin_matches(origin: &str, base: &str) -> bool {
    if origin == base {
        return true;
    }
    for prefix in ["http://localhost", "http://127.0.0.1"] {
        if !base.starts_with(prefix) {
            continue;
        }
        if let Some(rest) = origin.strip_prefix(prefix) {
            if rest.is_empty() {
                return true;
            }
            if let Some(port) = rest.strip_prefix(':')
                && !port.is_empty()
                && port.chars().all(|c| c.is_ascii_digit())
            {
                return true;
            }
        }
    }
    false
}

fn forbidden_response() -> Response {
    (
        StatusCode::FORBIDDEN,
        axum::Json(serde_json::json!({ "error": "forbidden" })),
    )
        .into_response()
}

/// Axum middleware. Non-`POST` methods pass through.
pub async fn origin_check_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    if req.method() != Method::POST {
        return next.run(req).await;
    }
    match assert_origin_allowed(&req, &state, allow_null_origin_from_env()) {
        Ok(()) => next.run(req).await,
        Err(resp) => *resp,
    }
}

#[cfg(test)]
#[path = "origin_check_tests.rs"]
mod tests;

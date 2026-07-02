//! Service-worker asset route.
//!
//! Reads the prebuilt `service-worker.js` from the web-root, replaces its
//! `let APP_VERSION = "...";` placeholder with the runtime version, and
//! returns the patched source. If the source file is missing or doesn't
//! contain the placeholder we don't silently 200 — we either inject a
//! fallback or return 500 so the broken state is visible.

use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use regex::Regex;
use std::sync::OnceLock;
use tokio::fs;

use crate::error::AppError;
use crate::state::AppState;

const APP_VERSION_REGEX_STR: &str = r#"let APP_VERSION = ".*?";"#;

/// Cache-busting headers attached to every service-worker response.
fn cache_bust_headers() -> [(HeaderName, &'static str); 4] {
    [
        (axum::http::header::CONTENT_TYPE, "application/javascript"),
        (
            axum::http::header::CACHE_CONTROL,
            "no-cache, no-store, must-revalidate",
        ),
        (axum::http::header::PRAGMA, "no-cache"),
        (axum::http::header::EXPIRES, "0"),
    ]
}

fn app_version_regex() -> &'static Regex {
    // `Regex::new` on a literal pattern is infallible after validation, so we
    // wrap it in a `OnceLock` to avoid the cost of recompiling per request.
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(APP_VERSION_REGEX_STR)
            .expect("APP_VERSION_REGEX_STR is a compile-time constant pattern")
    })
}

/// `GET /service-worker.js` — return the bundled service worker, with the
/// `APP_VERSION` placeholder replaced by the runtime version.
pub async fn serve_service_worker(State(state): State<AppState>) -> Result<Response, AppError> {
    let sw_path = state.web_root.join("service-worker.js");
    let content = fs::read_to_string(&sw_path).await.map_err(|e| {
        tracing::error!(
            target: "service_worker",
            path = %sw_path.display(),
            error = %e,
            "failed to read service-worker.js"
        );
        AppError::Io(e)
    })?;

    let replacement = format!(r#"let APP_VERSION = "{}";"#, state.config.version);
    let updated = match app_version_regex().replace(&content, replacement.as_str()) {
        // `Cow::Borrowed` means the regex never matched — the upstream
        // source lacks the placeholder. That's a release-blocking bug, so
        // log loudly and fall back to *appending* the assignment.
        std::borrow::Cow::Borrowed(_) => {
            tracing::warn!(
                target: "service_worker",
                "APP_VERSION placeholder missing; appending fallback assignment"
            );
            let fallback = format!("\nlet APP_VERSION = \"{}\";\n", state.config.version);
            format!("{content}{fallback}")
        }
        std::borrow::Cow::Owned(s) => s,
    };

    let mut headers = HeaderMap::new();
    for (name, value) in cache_bust_headers() {
        // `HeaderValue::from_str` only fails for embedded CTL chars or
        // non-visible bytes; the literals here are guaranteed safe.
        if let Ok(v) = HeaderValue::from_str(value) {
            headers.insert(name, v);
        }
    }
    Ok((StatusCode::OK, headers, updated).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_matches_simple_assignment() {
        let s = r#"let APP_VERSION = "1.2.3";"#;
        assert!(app_version_regex().is_match(s));
    }

    #[test]
    fn regex_replaces_assignment() {
        let s = r#"const x = 1; let APP_VERSION = "old"; other();"#;
        let replaced = app_version_regex().replace(s, r#"let APP_VERSION = "new";"#);
        assert!(replaced.contains(r#"let APP_VERSION = "new";"#));
        assert!(!replaced.contains(r#""old""#));
    }
}

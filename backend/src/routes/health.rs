//! Liveness probe.
//!
//! Returns a small JSON body with the current Unix epoch seconds. Used by
//! load balancers and the deployment smoke-tests; deliberately cheap (no
//! DB or filesystem access) so it can never deadlock under load.

use axum::Json;
use axum::response::IntoResponse;

/// `GET /health` → `{"status":"ok","timestamp":<unix-secs>}`.
pub async fn health_check() -> impl IntoResponse {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    tracing::debug!(target: "health", timestamp = secs, "health check");
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": secs
    }))
}

#[cfg(test)]
mod tests {
    // We don't directly unit-test the handler (axum's response types are
    // awkward outside an integration test), but the constants above are
    // the only side-effect-free path and are covered transitively by the
    // binary's integration suite.
}

//! Gate mutating authenticated endpoints behind a valid session.
//!
//! Installed via `middleware::from_fn_with_state(state.clone(), require_pin)`
//! on every route that needs an authenticated client.

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::CookieJar;

use crate::state::AppState;

use super::is_authenticated;

/// Reject the request if the client is not authenticated. Otherwise pass
/// through to the next handler.
pub async fn require_pin(
    jar: CookieJar,
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    if !is_authenticated(&jar, &state, req.headers()).await {
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({ "error": "Unauthorized" })),
        )
            .into_response();
    }
    next.run(req).await
}

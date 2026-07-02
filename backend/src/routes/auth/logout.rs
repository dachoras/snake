//! `POST /api/logout` — revoke the current session and clear the cookie.

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::CookieJar;

use super::COOKIE_NAME;
use super::cookie::build_clear_cookie;
use crate::state::AppState;

/// Clear the active session and replace the auth cookie with an expired
/// one. Idempotent: calling logout twice is a no-op the second time.
pub async fn logout(jar: CookieJar, State(state): State<AppState>) -> Response {
    if let Some(cookie) = jar.get(COOKIE_NAME) {
        state.revoke_session(cookie.value()).await;
    }
    let clear = build_clear_cookie(false);
    let jar = jar.add(clear);
    (jar, Json(serde_json::json!({ "success": true }))).into_response()
}

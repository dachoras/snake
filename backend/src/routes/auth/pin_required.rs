use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::IntoResponse,
};
use shared_backend::auth::attempts;
use shared_backend::server::get_client_ip;
use std::net::SocketAddr;
use std::time::Duration;

use crate::state::AppState;

pub async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "siteTitle": state.config.server.site_title,
        "baseUrl": state.config.server.base_url,
        "version": state.config.version,
        "enableTranslation": state.config.server.enable_translation,
        "enable_translation": state.config.server.enable_translation,
        "enableThemes": state.config.server.enable_themes,
        "enable_themes": state.config.server.enable_themes,
        "enablePrint": state.config.server.enable_print,
        "enable_print": state.config.server.enable_print,
        "showVersion": state.config.server.show_version,
        "show_version": state.config.server.show_version,
        "showGithub": state.config.server.show_github,
        "show_github": state.config.server.show_github,
    }))
}

pub async fn pin_required(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let ip_str = get_client_ip(
        &headers,
        addr,
        state.config.server.trust_proxy,
        &state.config.server.trusted_proxies,
    );
    let lockout_dur = Duration::from_secs(state.config.server.lockout_time_minutes * 60);
    axum::Json(serde_json::json!({
        "required": state.config.server.pin.is_some(),
        "length": state.config.server.pin.as_ref().map_or(0, |p| p.len()),
        "locked": attempts::is_locked_out(&ip_str, state.config.server.max_attempts, lockout_dur),
        "enable_translation": state.config.server.enable_translation,
        "enable_themes": state.config.server.enable_themes,
        "enable_print": state.config.server.enable_print,
        "show_version": state.config.server.show_version,
        "show_github": state.config.server.show_github,
    }))
}

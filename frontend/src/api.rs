//! HTTP API client for the Snake frontend.
//!
//! Wraps [`gloo_net::http::Request`] calls to the backend endpoints actually
//! exercised by the game UI: PIN-gated auth, configuration loading, and the
//! global leaderboard. Theme persistence lives on [`StorageService`] which
//! delegates to [`crate::storage`].

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::storage::StorageService as GenericStorage;
use shared_frontend::theme::{Theme, mapping::Scheme};

/// Theme persistence facade used by [`crate::app::App`].
///
/// Reads and writes the active theme to local storage (with cookie mirroring
/// handled inside [`crate::storage::StorageService`]). Unknown stored values
/// are normalised against the [`Scheme`] / [`Theme`] registry.
pub struct StorageService;

impl StorageService {
    /// Returns the canonical name of the user's saved theme.
    ///
    /// Accepts either a raw theme id (`"brinstar"`, `"tourian"`, ...) or the
    /// legacy scheme id stored by older clients; unknown values fall back to
    /// [`Theme::default`].
    pub fn get_theme() -> String {
        let raw = GenericStorage::get_item("theme", Theme::default().name());
        let theme = if let Some(scheme) = Scheme::from_id(&raw) {
            scheme.to_theme().name().to_string()
        } else {
            Theme::from_name(&raw)
                .unwrap_or_default()
                .name()
                .to_string()
        };
        if theme != raw {
            GenericStorage::set_item("theme", &theme);
        }
        theme
    }

    /// Persists the active theme name to local storage and the matching cookie.
    pub fn set_theme(theme: &str) {
        GenericStorage::set_item("theme", theme);
    }
}

/// REST endpoints exposed by the Snake backend.
pub struct ApiService;

/// One row of the global high-score table.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LeaderboardEntry {
    /// Player-chosen display name.
    pub name: String,
    /// Final score when the row was submitted.
    pub score: u32,
    /// ISO-8601 submission date set by the server; empty for outbound payloads.
    pub date: String,
}

/// Response payload of [`ApiService::check_pin_required`].
#[derive(Deserialize)]
pub struct PinRequiredResponse {
    /// `true` if the app is configured to require a PIN at startup.
    pub required: bool,
    /// Expected PIN length (in digits).
    pub length: usize,
    /// `true` if the IP is currently rate-limited and must wait.
    pub locked: bool,
}

/// Outbound payload of [`ApiService::verify_pin`].
#[derive(Serialize)]
pub struct VerifyPinPayload {
    /// PIN entered by the user; digits only.
    pub pin: String,
}

/// Response payload of [`ApiService::verify_pin`].
#[derive(Deserialize)]
pub struct VerifyPinResponse {
    /// `true` if the PIN was accepted.
    pub success: bool,
    /// Human-readable error message, surfaced to the login UI on failure.
    pub error: Option<String>,
}

/// Helper used by [`ConfigResponse`] so missing JSON fields default to `true`
/// (the production behaviour the backend ships with).
fn default_true() -> bool {
    true
}

/// Application-wide configuration fetched once at startup.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    /// Deployed backend version, surfaced in the header.
    pub version: String,
    /// Site title rendered in the header link.
    pub site_title: String,
    /// Whether the language picker is shown.
    #[serde(default)]
    pub enable_translation: bool,
    /// Whether the theme toggle is shown.
    #[serde(default = "default_true")]
    pub enable_themes: bool,
    /// Whether the print button is shown.
    #[serde(default = "default_true")]
    pub enable_print: bool,
    /// Whether the version badge is rendered in the header.
    #[serde(default = "default_true")]
    pub show_version: bool,
    /// Whether the GitHub footer link is rendered.
    #[serde(default = "default_true")]
    pub show_github: bool,
}

impl ApiService {
    /// Asks the backend whether a PIN gate is required to access the game.
    pub async fn check_pin_required() -> Result<PinRequiredResponse, gloo_net::Error> {
        Request::get("/api/pin-required")
            .send()
            .await?
            .json::<PinRequiredResponse>()
            .await
    }

    /// Submits the user's PIN. Translates HTTP 400/401/429 bodies with a
    /// `{"error": "..."}` shape into [`VerifyPinResponse`] with the error
    /// string preserved so the UI can render lockout hints.
    pub async fn verify_pin(pin: &str) -> Result<VerifyPinResponse, gloo_net::Error> {
        let payload = VerifyPinPayload {
            pin: pin.to_string(),
        };
        let response = Request::post("/api/verify-pin")
            .json(&payload)?
            .send()
            .await?;
        if (response.status() == 401 || response.status() == 429 || response.status() == 400)
            && let Ok(err_res) = response.json::<serde_json::Value>().await
            && let Some(err_str) = err_res.get("error").and_then(|v| v.as_str())
        {
            return Ok(VerifyPinResponse {
                success: false,
                error: Some(err_str.to_string()),
            });
        }
        response.json::<VerifyPinResponse>().await
    }

    /// Invalidates the server-side session cookie. Best-effort: any error is
    /// ignored because the client will optimistically transition to the
    /// logged-out state regardless.
    pub async fn logout() -> Result<(), gloo_net::Error> {
        Request::post("/api/logout").send().await?;
        Ok(())
    }

    /// Fetches the runtime configuration consumed by [`crate::app::App`].
    pub async fn get_config() -> Result<ConfigResponse, gloo_net::Error> {
        Request::get("/api/config")
            .send()
            .await?
            .json::<ConfigResponse>()
            .await
    }

    /// Fetches the global leaderboard, sorted server-side.
    pub async fn get_leaderboard() -> Result<Vec<LeaderboardEntry>, gloo_net::Error> {
        Request::get("/api/leaderboard")
            .send()
            .await?
            .json::<Vec<LeaderboardEntry>>()
            .await
    }

    /// Posts the player's final score and name. The server fills in the
    /// `date` field, so the outbound payload leaves it empty.
    pub async fn submit_score(name: &str, score: u32) -> Result<(), gloo_net::Error> {
        let payload = LeaderboardEntry {
            name: name.to_string(),
            score,
            date: String::new(),
        };
        let response = Request::post("/api/leaderboard")
            .json(&payload)?
            .send()
            .await?;
        if !response.ok() {
            let status = response.status();
            if let Ok(err_res) = response.json::<serde_json::Value>().await
                && let Some(err_str) = err_res.get("error").and_then(|v| v.as_str())
            {
                return Err(gloo_net::Error::GlooError(err_str.to_string()));
            }
            return Err(gloo_net::Error::GlooError(format!("HTTP error {status}")));
        }
        Ok(())
    }
}

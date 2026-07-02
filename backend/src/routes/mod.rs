pub mod auth;
pub mod leaderboard;
pub mod pages;

pub use auth::{get_config, logout, pin_required, rate_limit_middleware, require_pin, verify_pin};
pub use leaderboard::{get_leaderboard, submit_score};
pub use pages::{serve_login, serve_root};

use axum::{
    extract::State,
    response::IntoResponse,
};
use std::path::Path as StdPath;
use tokio::fs;

use crate::state::AppState;



// Service worker serving
pub async fn serve_service_worker(State(state): State<AppState>) -> impl IntoResponse {
    let sw_path = state
        .data_dir
        .parent()
        .unwrap()
        .join("frontend/dist/service-worker.js");
    match fs::read_to_string(&sw_path).await {
        Ok(content) => {
            let re = regex::Regex::new(r#"let APP_VERSION = ".*?";"#).unwrap();
            let replacement = format!(r#"let APP_VERSION = "{}";"#, state.config.version);
            let updated = re.replace(&content, replacement.as_str()).to_string();

            (
                [
                    (axum::http::header::CONTENT_TYPE, "application/javascript"),
                    (
                        axum::http::header::CACHE_CONTROL,
                        "no-cache, no-store, must-revalidate",
                    ),
                    (axum::http::header::PRAGMA, "no-cache"),
                    (axum::http::header::EXPIRES, "0"),
                ],
                updated,
            )
                .into_response()
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading service-worker.js: {}", e),
        )
            .into_response(),
    }
}

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    axum::Json(serde_json::json!({
        "status": "ok",
        "timestamp": secs
    }))
}

// Recursive file scanner for Web App/Assets manifest generation
fn get_files(dir: &StdPath, base_path: &str, files: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if file_name == ".DS_Store" || file_name == "Assets" {
                continue;
            }
            let sub_path = if base_path.is_empty() || base_path == "/" {
                format!("/{}", file_name)
            } else {
                format!("{}/{}", base_path, file_name)
            };
            if path.is_dir() {
                get_files(&path, &sub_path, files)?;
            } else {
                files.push(sub_path);
            }
        }
    }
    Ok(())
}

pub async fn serve_asset_manifest(State(_state): State<AppState>) -> impl IntoResponse {
    let public_dir = StdPath::new("frontend/dist");
    let mut files = Vec::new();
    if let Err(e) = get_files(public_dir, "", &mut files) {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error scanning assets: {}", e),
        )
            .into_response();
    }
    files.push("/asset-manifest.json".to_string());

    (
        [
            (axum::http::header::CONTENT_TYPE, "application/json"),
            (
                axum::http::header::CACHE_CONTROL,
                "no-cache, no-store, must-revalidate",
            ),
        ],
        axum::Json(files),
    )
        .into_response()
}

pub async fn serve_manifest(State(state): State<AppState>) -> impl IntoResponse {
    let manifest_path = StdPath::new("frontend/dist/Assets/manifest.json");
    let content = fs::read_to_string(&manifest_path).await.unwrap_or_else(|_| {
        r##"{
            "start_url": "/",
            "display": "standalone",
            "background_color": "#ffffff",
            "theme_color": "#000000",
            "icons": [
                {
                    "src": "logo.png",
                    "type": "image/png",
                    "sizes": "192x192"
                },
                {
                    "src": "logo.png",
                    "type": "image/png",
                    "sizes": "512x512"
                }
            ],
            "orientation": "any"
        }"##.to_string()
    });

    let mut val: serde_json::Value = serde_json::from_str(&content).unwrap_or_else(|_| {
        serde_json::json!({
            "start_url": "/",
            "display": "standalone",
            "background_color": "#ffffff",
            "theme_color": "#000000",
            "icons": [
                {
                    "src": "logo.png",
                    "type": "image/png",
                    "sizes": "192x192"
                },
                {
                    "src": "logo.png",
                    "type": "image/png",
                    "sizes": "512x512"
                }
            ],
            "orientation": "any"
        })
    });

    val["name"] = serde_json::Value::String(state.config.server.site_title.clone());
    val["short_name"] = serde_json::Value::String(state.config.server.site_title.clone());
    val["description"] = serde_json::Value::String("A traditional arcade snake game".to_string());

    (
        [
            (axum::http::header::CONTENT_TYPE, "application/json"),
            (
                axum::http::header::CACHE_CONTROL,
                "no-cache, no-store, must-revalidate",
            ),
        ],
        axum::Json(val),
    )
        .into_response()
}

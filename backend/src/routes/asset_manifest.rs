//! Static asset manifest endpoint.
//!
//! Generates `asset-manifest.json` on the fly by walking the web-root and
//! returning every file the frontend should precache (the PWA service
//! worker reads this list at install time).

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use std::path::Path;
use tokio::task;

use crate::error::AppError;
use crate::state::AppState;

/// File-system entry that should be skipped during the manifest scan.
const SKIP_NAMES: &[&str] = &[".DS_Store", "assets"];

/// Recursive scan of `dir`, returning URLs (relative to the web root) for
/// every regular file encountered.
///
/// `base_path` is the URL prefix to prepend; pass `""` for the root.
/// Returns [`std::io::Error`] on permission errors etc. so the caller can
/// surface them to the operator.
pub fn get_files(dir: &Path, base_path: &str, files: &mut Vec<String>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if SKIP_NAMES.contains(&file_name) {
            continue;
        }
        let sub_path = if base_path.is_empty() || base_path == "/" {
            format!("/{file_name}")
        } else {
            format!("{base_path}/{file_name}")
        };
        if path.is_dir() {
            get_files(&path, &sub_path, files)?;
        } else {
            files.push(sub_path);
        }
    }
    Ok(())
}

/// `GET /asset-manifest.json` — list of public asset paths for PWA precache.
pub async fn serve_asset_manifest(State(state): State<AppState>) -> Response {
    let web_root = state.web_root.clone();
    let scan = task::spawn_blocking(move || {
        let mut files = Vec::new();
        get_files(&web_root, "", &mut files)?;
        files.push("/asset-manifest.json".to_string());
        Ok::<_, std::io::Error>(files)
    })
    .await;

    match scan {
        Ok(Ok(files)) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            headers.insert(
                axum::http::header::CACHE_CONTROL,
                HeaderValue::from_static("no-cache, no-store, must-revalidate"),
            );
            (StatusCode::OK, headers, Json(files)).into_response()
        }
        Ok(Err(e)) => {
            tracing::error!(target: "asset_manifest", error = %e, "asset scan failed");
            AppError::Io(e).into_response()
        }
        Err(e) => {
            tracing::error!(target: "asset_manifest", error = %e, "asset scan task panicked");
            AppError::internal("asset scan task panicked").into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn get_files_skips_ds_store_and_assets() {
        let tmp = TempDir::new().expect("tempdir");
        std::fs::write(tmp.path().join("index.html"), b"<html>").expect("write");
        std::fs::write(tmp.path().join(".DS_Store"), b"junk").expect("write");
        std::fs::create_dir(tmp.path().join("assets")).expect("mkdir");
        std::fs::write(tmp.path().join("assets").join("logo.png"), b"\x89").expect("write");

        let mut files = Vec::new();
        get_files(tmp.path(), "", &mut files).expect("scan");
        assert!(files.contains(&"/index.html".to_string()));
        assert!(!files.iter().any(|f| f.contains(".DS_Store")));
        assert!(!files.iter().any(|f| f.contains("assets")));
    }

    #[test]
    fn get_files_returns_empty_for_missing_dir() {
        let mut files = Vec::new();
        get_files(Path::new("/nonexistent-opencode-dir-xyz"), "", &mut files).expect("ok");
        assert!(files.is_empty());
    }

    #[test]
    fn get_files_walks_nested_directories() {
        let tmp = TempDir::new().expect("tempdir");
        let nested = tmp.path().join("static").join("js");
        std::fs::create_dir_all(&nested).expect("mkdir");
        std::fs::write(nested.join("app.js"), b"console.log(1)").expect("write");

        let mut files = Vec::new();
        get_files(tmp.path(), "", &mut files).expect("scan");
        assert!(files.iter().any(|f| f == "/static/js/app.js"));
    }
}

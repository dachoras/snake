//! Path resolution helpers.
//!
//! Centralises the logic for figuring out where the data directory lives
//! relative to the running binary, so handlers never have to call
//! `Path::parent().unwrap()` (which panics when the data dir is the
//! filesystem root).

use std::path::{Path, PathBuf};

/// Default relative path of the on-disk data directory.
pub const DEFAULT_DATA_DIR: &str = "data";

/// Default relative path of the prebuilt frontend bundle.
pub const DEFAULT_FRONTEND_DIR: &str = "frontend/dist";

/// Env-var name overriding [`DEFAULT_DATA_DIR`].
pub const DATA_DIR_ENV: &str = "SNAKE_DATA_DIR";

/// Env-var name overriding [`DEFAULT_FRONTEND_DIR`].
pub const FRONTEND_DIR_ENV: &str = "SNAKE_FRONTEND_DIR";

/// Resolve a directory path from an env-var override (if non-empty), or
/// fall back to the supplied default.
///
/// Whitespace-only env values are ignored so a stray `\n` from a mis-piped
/// shell doesn't accidentally disable the override.
#[must_use]
pub fn resolve_dir(env_value: Option<&str>, default: &str) -> PathBuf {
    if let Some(v) = env_value
        && !v.trim().is_empty()
    {
        return PathBuf::from(v.trim());
    }
    PathBuf::from(default)
}

/// Resolve the data directory the application should use.
///
/// Reads [`DATA_DIR_ENV`] from the process environment; falls back to
/// [`DEFAULT_DATA_DIR`] when the var is unset or whitespace-only.
///
/// The returned path is **not** required to exist; callers must call
/// [`crate::state::AppStateInner::ensure_data_dir`] before touching it.
#[must_use]
pub fn resolve_data_dir() -> PathBuf {
    resolve_dir(
        std::env::var(DATA_DIR_ENV).ok().as_deref(),
        DEFAULT_DATA_DIR,
    )
}

/// Resolve the web root (prebuilt frontend bundle) the application should
/// serve.
///
/// Reads [`FRONTEND_DIR_ENV`] from the process environment; falls back to
/// [`DEFAULT_FRONTEND_DIR`] when the var is unset or whitespace-only.
///
/// The returned path is **not** required to exist.
#[must_use]
pub fn resolve_frontend_dir() -> PathBuf {
    resolve_dir(
        std::env::var(FRONTEND_DIR_ENV).ok().as_deref(),
        DEFAULT_FRONTEND_DIR,
    )
}

/// Return the parent of `p`, falling back to `"."` when `p` has no parent
/// or its parent is empty (filesystem root or a bare relative segment).
#[must_use]
pub fn safe_parent(p: &Path) -> PathBuf {
    match p.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

/// Leaderboard JSON path under the resolved data directory.
#[must_use]
pub fn leaderboard_file(data_dir: &Path) -> PathBuf {
    data_dir.join("leaderboard.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn safe_parent_handles_filesystem_root() {
        let p = Path::new("/");
        assert_eq!(safe_parent(p), PathBuf::from("."));
    }

    #[test]
    fn safe_parent_handles_single_segment() {
        let p = Path::new("data");
        assert_eq!(safe_parent(p), PathBuf::from("."));
    }

    #[test]
    fn safe_parent_preserves_nested_paths() {
        let p = Path::new("a/b/c");
        assert_eq!(safe_parent(p), PathBuf::from("a/b"));
    }

    #[test]
    fn leaderboard_file_lives_under_data_dir() {
        let dir = TempDir::new().expect("tempdir");
        let path = leaderboard_file(dir.path());
        assert_eq!(
            path.file_name().and_then(|s| s.to_str()),
            Some("leaderboard.json")
        );
        assert_eq!(path.parent(), Some(dir.path()));
    }

    #[test]
    fn resolve_dir_falls_back_to_default() {
        let resolved = resolve_dir(None, DEFAULT_DATA_DIR);
        assert_eq!(resolved, PathBuf::from(DEFAULT_DATA_DIR));
        let resolved = resolve_dir(Some(""), DEFAULT_DATA_DIR);
        assert_eq!(resolved, PathBuf::from(DEFAULT_DATA_DIR));
        let resolved = resolve_dir(Some("   "), DEFAULT_DATA_DIR);
        assert_eq!(resolved, PathBuf::from(DEFAULT_DATA_DIR));
    }

    #[test]
    fn resolve_dir_honours_override() {
        let resolved = resolve_dir(Some("/tmp/opencode-data"), DEFAULT_DATA_DIR);
        assert_eq!(resolved, PathBuf::from("/tmp/opencode-data"));
    }

    #[test]
    fn resolve_dir_trims_whitespace() {
        let resolved = resolve_dir(Some("   /var/snake   "), DEFAULT_DATA_DIR);
        assert_eq!(resolved, PathBuf::from("/var/snake"));
    }
}

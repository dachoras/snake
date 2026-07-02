//! Helpers for installing the global `tracing` subscriber.
//!
//! Always installs a stderr `fmt` layer. When `log_dir` is `Some`, also
//! appends two file layers (`error.log`, `app.log`) with appropriate level
//! filters. Errors opening the files are logged but never fatal — we'd
//! rather lose log files than crash on startup.

use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Env-var name controlling where structured log files live.
pub const LOG_DIR_ENV: &str = "LOG_DIR";

/// Pick a default log directory based on whether `/app/data` exists.
///
/// Used only when [`LOG_DIR_ENV`] is unset; explicitly setting `LOG_DIR`
/// always wins.
#[must_use]
pub fn default_log_dir() -> Option<String> {
    if std::path::Path::new("/app/data").is_dir() {
        Some("/app/data/log".to_string())
    } else {
        Some("/app/log".to_string())
    }
}

/// Normalise a `LOG_DIR` env value into a real directory or `None` for off.
///
/// `LOG_DIR=off`, `none`, or `false` (case-insensitive, whitespace-trimmed)
/// disables file logging. Anything else is treated as a directory path.
#[must_use]
pub fn normalise_log_dir(raw: Option<String>) -> Option<String> {
    let value = raw?;
    let trimmed = value.trim();
    let lowered = trimmed.to_ascii_lowercase();
    let disabled = matches!(lowered.as_str(), "" | "off" | "none" | "false");
    if disabled {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Open `dir/name` for append, returning `None` on failure rather than
/// crashing startup. Errors are logged at WARN level.
fn open_log_file(dir: &str, name: &str) -> Option<std::fs::File> {
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(std::path::Path::new(dir).join(name))
        .map_err(|e| {
            tracing::warn!(
                target: "bootstrap",
                file = name,
                error = %e,
                "could not open log file"
            );
            e
        })
        .ok()
}

/// Install the global `tracing` subscriber. See module docs.
pub fn init_tracing(log_dir: Option<&str>) {
    let (error_layer, app_layer) = match log_dir {
        Some(dir) => {
            if let Err(e) = std::fs::create_dir_all(dir) {
                tracing::warn!(
                    target: "bootstrap",
                    dir,
                    error = %e,
                    "could not create log directory; file logging disabled"
                );
                (None, None)
            } else {
                let err_layer = open_log_file(dir, "error.log").map(|file| {
                    tracing_subscriber::fmt::layer()
                        .with_writer(std::sync::Mutex::new(file))
                        .with_ansi(false)
                        .with_filter(tracing_subscriber::filter::LevelFilter::WARN)
                });
                let app_layer = open_log_file(dir, "app.log").map(|file| {
                    tracing_subscriber::fmt::layer()
                        .with_writer(std::sync::Mutex::new(file))
                        .with_ansi(false)
                        .with_filter(tracing_subscriber::filter::LevelFilter::INFO)
                });
                (err_layer, app_layer)
            }
        }
        None => (None, None),
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(error_layer)
        .with(app_layer)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise_log_dir_handles_off_variants() {
        assert_eq!(normalise_log_dir(Some("off".to_string())), None);
        assert_eq!(normalise_log_dir(Some("OFF".to_string())), None);
        assert_eq!(normalise_log_dir(Some(" none ".to_string())), None);
        assert_eq!(normalise_log_dir(Some("False".to_string())), None);
    }

    #[test]
    fn normalise_log_dir_passes_through_real_path() {
        assert_eq!(
            normalise_log_dir(Some("/var/log/snake".to_string())),
            Some("/var/log/snake".to_string())
        );
        assert_eq!(
            normalise_log_dir(Some("  /tmp/log  ".to_string())),
            Some("/tmp/log".to_string())
        );
    }

    #[test]
    fn normalise_log_dir_handles_none_and_empty() {
        assert_eq!(normalise_log_dir(None), None);
        assert_eq!(normalise_log_dir(Some("   ".to_string())), None);
        assert_eq!(normalise_log_dir(Some(String::new())), None);
    }
}

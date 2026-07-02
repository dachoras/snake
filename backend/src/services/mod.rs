//! Cross-cutting service helpers.
//!
//! Modules:
//! - [`paths`] — resolve data/web roots without panicking on `/`
//! - [`rate_limit`] — per-IP request-budget sliding window
//! - [`session`] — cryptographically random session ids

pub mod paths;
pub mod rate_limit;
pub mod session;

pub use paths::{
    DEFAULT_DATA_DIR, DEFAULT_FRONTEND_DIR, leaderboard_file, resolve_data_dir,
    resolve_frontend_dir, safe_parent,
};
pub use rate_limit::{DEFAULT_MAX_REQUESTS, DEFAULT_WINDOW, RateLimiter};
pub use session::generate_session_id;

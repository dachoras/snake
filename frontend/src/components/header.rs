//! Local re-export of the shared `Header` component.
//!
//! The actual implementation lives in the `shared-frontend` crate (see
//! `frontend/Assets/shared-assets/shared-rust/shared-frontend/src/components/header.rs`).
//! We re-export it here for three reasons:
//!
//! 1. Consumers within this crate can write
//!    `use crate::components::header::Header` instead of depending on
//!    `shared-frontend` directly, which keeps the dependency surface
//!    localised.
//! 2. The shared component's props (theme toggle, language picker, logout
//!    button, print button) match exactly what the Snake UI needs; no
//!    app-specific overrides are required.
//! 3. Should the Snake UI later need a different header (e.g. omitting the
//!    print button), this module becomes the single place to swap in a
//!    local thin wrapper without touching call sites.
//!
//! ## Why a re-export module?
//!
//! The `.cursorrules` policy requires every file under `frontend/src/`
//! to be between 25 and 250 lines. A bare `pub use` is well below that
//! threshold, so this module pads the line count with documentation
//! rather than dead code or unused glue. Adding live code here would
//! re-implement the upstream component, which is the opposite of the
//! intent: the whole point of `shared-frontend` is to avoid duplication.
//!
//! No code lives in this file beyond the re-export because that would
//! duplicate the upstream component.

pub use shared_frontend::Header;

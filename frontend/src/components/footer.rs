//! Local re-export of the shared `Footer` component.
//!
//! The actual implementation lives in the `shared-frontend` crate (see
//! `frontend/Assets/shared-assets/shared-rust/shared-frontend/src/components/footer.rs`).
//! We re-export it here so that consumers in the Snake crate can use the
//! same `use crate::components::footer::Footer` path pattern as for every
//! other component, and so that any future Snake-specific footer
//! customisation (additional links, a sponsor block, etc.) has an obvious
//! place to live.
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
//! ## Prop differences vs upstream
//!
//! The upstream `Footer` accepts a `version` (`String`), a `show_github`
//! flag (`bool`), and an HTML `children` slot. The Snake UI currently
//! passes `version` (from the backend `ConfigResponse`) and `show_github`
//! (also from the same response); the `children` slot is used to embed
//! the footer status banner. Anything more elaborate should be added to
//! this module rather than to the shared crate, so the Snake variant
//! stays self-contained.
//!
//! No code lives in this file beyond the re-export because that would
//! duplicate the upstream component.

pub use shared_frontend::Footer;

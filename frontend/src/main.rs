//! Snake frontend crate root.
//!
//! Wires together the application modules and bootstraps the Yew client
//! renderer. The crate is intentionally small at the top level: every
//! non-trivial concern (components, state, API, i18n, storage) lives in its
//! own sibling module so that this file stays as a thin shell whose sole job
//! is to mount [`app::App`] into the DOM element provided by `index.html`.
//!
//! ## Module map
//!
//! - [`api`] — backend HTTP client (`ApiService`) and theme persistence
//!   facade (`StorageService`).
//! - [`app`] — root `App` component (state machine, view, message handlers).
//! - [`components`] — reusable Yew components (board, dpad, leaderboard,
//!   login, header/footer re-exports, etc.).
//! - [`i18n`] — locale detection, the per-language string tables, and the
//!   `LocaleContext` provider consumed throughout the UI.
//! - [`storage`] — `localStorage` and `document.cookie` access layer.
//!
//! Nothing in this crate runs server-side; everything executes in the
//! browser after the WASM bundle is loaded, so [`web_sys::window`] is
//! always available inside components and hooks.

mod api;
mod app;
mod components;
mod i18n;
mod storage;

use app::App;

/// Bootstraps the Yew client renderer and mounts [`App`] into the body.
///
/// Yew's [`yew::Renderer`] targets the `<body>` element by default, which
/// matches the markup in `index.html`.
fn main() {
    yew::Renderer::<App>::new().render();
}

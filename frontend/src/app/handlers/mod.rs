//! `App` message-handler helpers, grouped by responsibility.
//!
//! Each submodule exposes one or more `impl App` methods. The split keeps
//! every file under the 250-line ceiling enforced by `.cursorrules` and
//! makes it easy to reason about each handler in isolation.
//!
//! - [`config`] — startup responses from the backend.
//! - [`auth`] — login / logout flows.
//! - [`ui`] — user-driven UI updates (theme, status banner, print).
//! - [`system`] — environment events (connectivity, language).

pub mod auth;
pub mod config;
pub mod system;
pub mod ui;

use crate::app::Msg;
use yew::prelude::*;

/// Pushes a transient status banner and clears it after 3 seconds.
///
/// Centralises the timer dance that previously appeared four times across
/// the auth, theme, online, and print handlers in the original monolithic
/// `update` function.
pub fn show_temporary_status(ctx: &Context<crate::app::App>, msg: &str, level: &str) {
    ctx.link()
        .send_message(Msg::SetStatus(Some((msg.to_string(), level.to_string()))));
    let link = ctx.link().clone();
    gloo_timers::callback::Timeout::new(3000, move || {
        link.send_message(Msg::SetStatus(None));
    })
    .forget();
}

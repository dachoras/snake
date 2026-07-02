//! Handlers for authentication state transitions.
//!
//! The two messages here are [`Msg::SetAuthenticated`] (driven by the
//! `Login` component or by an internal `Logout` follow-up) and [`Msg::Logout`]
//! (driven by the header logout button).

use crate::api::ApiService;
use crate::app::{App, Msg};
use shared_frontend::i18n::Language;
use shared_frontend::i18n::strings::{StringKey, lookup};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::show_temporary_status;

impl App {
    /// Updates the authenticated flag and surfaces a localised status banner.
    ///
    /// On successful authentication we also pre-warm the leaderboard cache
    /// so the [`SnakeGame`](crate::components::snake_game::SnakeGame) view
    /// doesn't show the empty-state placeholder on first render.
    pub fn handle_set_authenticated(&mut self, ctx: &Context<Self>, auth: bool) -> bool {
        self.authenticated = auth;
        let lang = Language::from_code(&self.locale_state);
        if auth {
            let pin_success = lookup(StringKey::StatusPinSuccess, lang).to_string();
            show_temporary_status(ctx, &pin_success, "success");

            let link = ctx.link().clone();
            spawn_local(async move {
                let _ = ApiService::get_leaderboard().await;
                // The pre-warm result is intentionally discarded: the
                // snake_state hook fetches the leaderboard itself on mount,
                // and any later update is propagated via that fetch.
                let _ = link;
            });
        } else {
            let logout_msg = lookup(StringKey::StatusLogout, lang).to_string();
            show_temporary_status(ctx, &logout_msg, "success");
        }
        true
    }

    /// Fires the logout request and, on success, demotes the user to the
    /// login screen by chaining a [`Msg::SetAuthenticated(false)`] dispatch.
    pub fn handle_logout(&mut self, ctx: &Context<Self>) -> bool {
        let link = ctx.link().clone();
        spawn_local(async move {
            if ApiService::logout().await.is_ok() {
                link.send_message(Msg::SetAuthenticated(false));
            }
        });
        // No re-render: the logout flow resolves asynchronously.
        false
    }
}

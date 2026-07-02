//! Handlers for environment-driven messages.
//!
//! Connectivity changes (browser `online` / `offline` events) and locale
//! changes (header language picker) are processed here.

use crate::app::App;
use shared_frontend::i18n::Language;
use shared_frontend::i18n::strings::{StringKey, lookup};
use yew::prelude::*;

use super::show_temporary_status;

impl App {
    /// Persists the new locale both in `localStorage` and in component state.
    pub fn handle_switch_language(&mut self, _ctx: &Context<Self>, lang: String) -> bool {
        crate::i18n::set_saved_locale(&lang);
        self.locale_state = lang;
        true
    }

    /// Surfaces a localised status banner when connectivity changes.
    pub fn handle_online_status_changed(&mut self, ctx: &Context<Self>, online: bool) -> bool {
        let lang = Language::from_code(&self.locale_state);
        let (msg_key, cls) = if online {
            (StringKey::StatusOnline, "success")
        } else {
            (StringKey::StatusOffline, "error")
        };
        let status_msg = lookup(msg_key, lang).to_string();
        show_temporary_status(ctx, &status_msg, cls);
        true
    }
}

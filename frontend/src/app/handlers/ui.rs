//! Handlers for purely UI-driven messages.
//!
//! Theme cycling, transient status updates, and the print button all live
//! here. They share the [`show_temporary_status`](super::show_temporary_status)
//! helper to flash a banner in the footer.

use crate::api::StorageService;
use crate::app::App;
use shared_frontend::i18n::Language;
use shared_frontend::i18n::strings::{StringKey, lookup};
use shared_frontend::theme::Theme;
use yew::prelude::*;

use super::show_temporary_status;

impl App {
    /// Cycles through the canonical theme order and persists the new value.
    pub fn handle_toggle_theme(&mut self, ctx: &Context<Self>) -> bool {
        let current = Theme::from_name(&self.theme).unwrap_or_default();
        let next = match current {
            Theme::Brinstar => Theme::Norfair,
            Theme::Norfair => Theme::WreckedShip,
            Theme::WreckedShip => Theme::Maridia,
            Theme::Maridia => Theme::Tourian,
            Theme::Tourian => Theme::Crateria,
            Theme::Crateria => Theme::Brinstar,
        };
        StorageService::set_theme(next.name());
        if let Some(html) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = html.set_attribute("data-theme", next.name());
            let _ = html.set_attribute("class", next.name());
        }
        self.theme = next.name().to_string();

        let lang = Language::from_code(&self.locale_state);
        let theme_msg = lookup(StringKey::StatusThemeChanged, lang).to_string();
        show_temporary_status(ctx, &theme_msg, "success");
        true
    }

    /// Stores the (message, css_class) pair for the footer banner. A `None`
    /// payload clears the banner.
    pub fn handle_set_status(
        &mut self,
        _ctx: &Context<Self>,
        status: Option<(String, String)>,
    ) -> bool {
        self.active_notification = status;
        true
    }

    /// Invokes `window.print()` and reports success/failure in the footer
    /// banner. The actual print dialog is browser-controlled; we only react
    /// to whether the API call itself returned an error.
    pub fn handle_print(&mut self, ctx: &Context<Self>) -> bool {
        if let Some(window) = web_sys::window() {
            let print_res = window.print();
            let lang = Language::from_code(&self.locale_state);
            let (msg_key, cls) = if print_res.is_ok() {
                (StringKey::StatusPrintSuccess, "success")
            } else {
                (StringKey::StatusPrintFailure, "error")
            };
            let status_msg = lookup(msg_key, lang).to_string();
            show_temporary_status(ctx, &status_msg, cls);
        }
        // Print is fire-and-forget; no component state changes.
        false
    }
}

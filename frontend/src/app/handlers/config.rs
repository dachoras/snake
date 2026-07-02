//! Handlers for startup configuration messages.
//!
//! Both [`Msg::LoadConfig`] and [`Msg::LoadPinRequired`] are dispatched in
//! response to the initial `get_config` / `check_pin_required` probes issued
//! by [`super::super::update::App::create_app`].

use crate::api::{ApiService, StorageService};
use crate::app::{App, Msg};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

impl App {
    /// Applies backend configuration: version, site title, feature toggles,
    /// theme fallback, and document title.
    pub fn handle_load_config(
        &mut self,
        _ctx: &Context<Self>,
        config: crate::api::ConfigResponse,
    ) -> bool {
        self.app_version = config.version;
        self.site_title = config.site_title.clone();
        self.enable_translation = config.enable_translation;
        self.enable_themes = config.enable_themes;
        self.enable_print = config.enable_print;
        self.show_version = config.show_version;
        self.show_github = config.show_github;

        // When themes are disabled by the backend, force the canonical
        // "tourian" theme and propagate it to the <html> element so the
        // stylesheet selector matches.
        if !config.enable_themes {
            self.theme = "tourian".to_string();
            StorageService::set_theme("tourian");
            if let Some(html) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.document_element())
            {
                let _ = html.set_attribute("data-theme", "tourian");
                let _ = html.set_attribute("class", "tourian");
            }
        }

        // Mirror the site title into the browser tab. No-op if the document
        // isn't available (it always is in the renderer, but defensive).
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            doc.set_title(&config.site_title);
        }

        true
    }

    /// Records whether the backend considers a PIN gate mandatory.
    pub fn handle_load_pin_required(&mut self, _ctx: &Context<Self>, required: bool) -> bool {
        self.is_pin_required = required;
        true
    }

    /// Helper used by `create_app`: issues the two startup probes and pipes
    /// their results back through the component link.
    pub fn spawn_startup_probes(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        spawn_local(async move {
            if let Ok(config) = ApiService::get_config().await {
                link.send_message(Msg::LoadConfig(config));
            }
        });

        let link = ctx.link().clone();
        spawn_local(async move {
            if let Ok(res) = ApiService::check_pin_required().await {
                link.send_message(Msg::LoadPinRequired(res.required));
            }
        });
    }
}

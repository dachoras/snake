//! Root application component and message model.
//!
//! [`App`] is the single top-level Yew component mounted by [`crate::main`].
//! It owns the global state machine (theme, locale, online status, auth) and
//! drives all other components via props and the [`LocaleContext`]
//! (re-exported from [`crate::i18n`]).
//!
//! Message handling is split into focused `impl App` methods defined in
//! [`update`]; the `view` lives in [`view`]. Submodules are kept small (≤250
//! lines each) to preserve compile throughput on the WASM target.

pub mod handlers;
pub mod update;
pub mod view;

use crate::api::ConfigResponse;
use yew::prelude::*;

/// Messages dispatched by child components and DOM event listeners.
///
/// Each variant corresponds to a discrete user-visible transition or
/// network-driven update; the handler methods on `App` (in [`update`])
/// keep each arm small enough to stay under the
/// [`clippy::cognitive-complexity-threshold`](../../../../clippy.toml).
pub enum Msg {
    /// Backend configuration arrived from [`crate::api::ApiService::get_config`].
    LoadConfig(ConfigResponse),
    /// Backend answered whether a PIN gate is required.
    LoadPinRequired(bool),
    /// Auth state changed (login success, logout, programmatic unlock).
    SetAuthenticated(bool),
    /// User picked a new locale in the header language picker.
    SwitchLanguage(String),
    /// User clicked the theme toggle.
    ToggleTheme,
    /// User clicked the logout button.
    Logout,
    /// Footer status banner text and severity level.
    SetStatus(Option<(String, String)>),
    /// Browser fired `online` / `offline` events.
    OnlineStatusChanged(bool),
    /// User clicked the print button.
    Print,
}

/// Top-level application state.
///
/// All fields are populated by [`update::App::create_app`] and the message
/// handlers defined in [`update`]. The defaults are conservative: the
/// `app_version` and `site_title` are empty until the `/api/config` response
/// arrives so the UI never flashes a stale string.
pub struct App {
    /// `true` once the user has cleared the PIN gate (or none was required).
    pub authenticated: bool,
    /// Backend version string; `""` until [`Msg::LoadConfig`] resolves.
    pub app_version: String,
    /// Site title shown in the header; `""` until [`Msg::LoadConfig`] resolves.
    pub site_title: String,
    /// Canonical theme name (e.g. `"brinstar"`, `"tourian"`).
    pub theme: String,
    /// Current locale code (`"en"`, `"de"`, ...).
    pub locale_state: String,
    /// Optional `(message, css_class)` for the footer's transient status banner.
    pub active_notification: Option<(String, String)>,
    /// Whether the PIN gate is required to access the game.
    pub is_pin_required: bool,
    /// Show the language picker in the header.
    pub enable_translation: bool,
    /// Show the theme toggle in the header.
    pub enable_themes: bool,
    /// Show the print button in the header.
    pub enable_print: bool,
    /// Show the version badge in the header.
    pub show_version: bool,
    /// Show the GitHub link in the footer.
    pub show_github: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::create_app(ctx)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.update_app(ctx, msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_app(ctx)
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            use wasm_bindgen::JsCast;
            // The renderer only runs in the browser, so `window()` is safe to
            // unwrap. Documented per the "no unwrap in non-test code" rule.
            let window = web_sys::window().expect("renderer runs in a browser window");

            let link_online = ctx.link().clone();
            let on_online =
                wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                    link_online.send_message(Msg::OnlineStatusChanged(true));
                });
            window
                .add_event_listener_with_callback("online", on_online.as_ref().unchecked_ref())
                .expect("failed to register online listener");
            on_online.forget();

            let link_offline = ctx.link().clone();
            let on_offline =
                wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                    link_offline.send_message(Msg::OnlineStatusChanged(false));
                });
            window
                .add_event_listener_with_callback("offline", on_offline.as_ref().unchecked_ref())
                .expect("failed to register offline listener");
            on_offline.forget();
        }
    }
}

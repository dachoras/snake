//! Internationalisation plumbing for the Snake UI.
//!
//! Three responsibilities:
//!
//! 1. Detect the user's preferred locale from `navigator.language`
//!    ([`detect_browser_locale`]) and persist the choice in `localStorage`
//!    via [`crate::storage::StorageService`] ([`get_saved_locale`] /
//!    [`set_saved_locale`]).
//! 2. Expose a Yew context ([`LocaleContext`]) so any component can call
//!    `ctx.t("score")` without threading the locale code through props.
//! 3. Dispatch a translation key to the matching language table via
//!    [`translate`], falling back to the raw key when the table has no entry.

use yew::prelude::*;

mod de;
mod en;
mod es;
mod fr;
mod ja;
mod pt;
mod ru;
mod zh;

/// Shared context object provided by the root [`crate::app::App`] view.
///
/// Components deeper in the tree call `use_context::<LocaleContext>()` to
/// access the current locale and a callback for switching it from a child.
#[derive(Clone, PartialEq)]
pub struct LocaleContext {
    /// Active locale code (`"en"`, `"de"`, ...).
    pub current: String,
    /// Dispatched when the user picks a new language from the picker.
    pub on_change: Callback<String>,
}

impl LocaleContext {
    /// Convenience wrapper around [`translate`] using `self.current`.
    pub fn t(&self, key: &str) -> String {
        translate(&self.current, key)
    }
}

/// Inspects `window.navigator.language` and maps it onto one of the locale
/// codes supported by this crate. Returns `"en"` when the navigator API is
/// unavailable or the language is unknown.
pub fn detect_browser_locale() -> String {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        if let Some(lang) = navigator.language() {
            let l = lang.to_lowercase();
            if l.starts_with("zh") {
                return "zh".to_string();
            }
            if l.starts_with("es") {
                return "es".to_string();
            }
            if l.starts_with("de") {
                return "de".to_string();
            }
            if l.starts_with("ja") {
                return "ja".to_string();
            }
            if l.starts_with("fr") {
                return "fr".to_string();
            }
            if l.starts_with("pt") {
                return "pt".to_string();
            }
            if l.starts_with("ru") {
                return "ru".to_string();
            }
        }
    }
    "en".to_string()
}

/// Reads the saved locale from `localStorage`, falling back to
/// [`detect_browser_locale`] when nothing has been persisted yet.
pub fn get_saved_locale() -> String {
    crate::storage::StorageService::get_item("lang", &detect_browser_locale())
}

/// Persists the user's locale choice in `localStorage`.
pub fn set_saved_locale(locale: &str) {
    crate::storage::StorageService::set_item("lang", locale);
}

/// Looks up a translation key in the appropriate language table.
///
/// Falls back to the raw key when no table contains it so the UI never
/// goes blank for an untranslated string — the developer sees the key in
/// place during development.
pub fn translate(lang: &str, key: &str) -> String {
    let l = if lang.starts_with("zh") {
        "zh"
    } else if lang.starts_with("es") {
        "es"
    } else if lang.starts_with("de") {
        "de"
    } else if lang.starts_with("ja") {
        "ja"
    } else if lang.starts_with("fr") {
        "fr"
    } else if lang.starts_with("pt") {
        "pt"
    } else if lang.starts_with("ru") {
        "ru"
    } else {
        "en"
    };

    let val = match l {
        "zh" => zh::translate(key),
        "es" => es::translate(key),
        "de" => de::translate(key),
        "ja" => ja::translate(key),
        "fr" => fr::translate(key),
        "pt" => pt::translate(key),
        "ru" => ru::translate(key),
        _ => en::translate(key),
    };

    val.unwrap_or(key).to_string()
}

//! Keyboard input handling for the Snake game.
//!
//! Installs a `keydown` listener on `window` for the lifetime of the
//! effect's current `(started, game_over, paused)` tuple. Keys are mapped
//! to direction deltas by [`direction_for_key`]; Escape / P toggles the
//! pause overlay regardless of the dpad state.

use yew::prelude::*;

use crate::components::event_listener::EventListener;

/// Maps a [`KeyboardEvent::key`](web_sys::KeyboardEvent::key) value to a
/// direction delta.
///
/// Returns `None` for keys that aren't directional inputs (anything that
/// isn't an arrow key or a WASD letter, case-insensitive). Centralising
/// this lookup means the keyboard handler and any future input adapter
/// (gamepad, touch swipe, ...) share the exact same mapping.
pub fn direction_for_key(key: &str) -> Option<(i32, i32)> {
    match key {
        "ArrowUp" | "w" | "W" => Some((0, -1)),
        "ArrowDown" | "s" | "S" => Some((0, 1)),
        "ArrowLeft" | "a" | "A" => Some((-1, 0)),
        "ArrowRight" | "d" | "D" => Some((1, 0)),
        _ => None,
    }
}

/// Installs the keyboard listener, tearing it down on re-run or unmount.
///
/// The listener:
/// 1. Toggles the pause overlay when Escape / P is pressed (only while a
///    game is in progress).
/// 2. Ignores movement keys while the game is paused.
/// 3. Forwards every recognised direction key to `on_dpad_press`.
#[hook]
pub fn use_keyboard_listener(
    started: &UseStateHandle<bool>,
    game_over: &UseStateHandle<bool>,
    paused: &UseStateHandle<bool>,
    on_dpad_press: &Callback<(i32, i32)>,
) {
    let is_started = **started;
    let is_game_over = **game_over;
    let is_paused = **paused;
    let paused = paused.clone();

    // Store the latest dpad press callback in a ref to avoid stale closure capture
    let callback_ref = use_mut_ref(|| on_dpad_press.clone());
    *callback_ref.borrow_mut() = on_dpad_press.clone();

    let callback_ref_for_listener = callback_ref.clone();

    use_effect_with(
        (is_started, is_game_over, is_paused),
        move |&(st, go, ps)| {
            // The renderer only runs in a browser window, so `window()` is
            // safe to unwrap. Documented per the "no unwrap in non-test code"
            // rule that applies to this crate.
            let window = web_sys::window().expect("renderer runs in a browser window");
            let paused = paused.clone();
            let callback_ref = callback_ref_for_listener.clone();
            let listener = EventListener::new(&window, "keydown", move |e: web_sys::Event| {
                // The event is registered as `"keydown"` so the target is
                // always a `KeyboardEvent`; the cast cannot fail at runtime.
                use wasm_bindgen::JsCast;
                let key_event = e
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .expect("keydown event is a KeyboardEvent");
                let key = key_event.key();

                if key == "Escape" || key == "p" || key == "P" {
                    if st && !go {
                        paused.set(!ps);
                    }
                    return;
                }

                // Disallow movement inputs while paused.
                if ps {
                    return;
                }

                if let Some(dir) = direction_for_key(&key) {
                    callback_ref.borrow().emit(dir);
                }
            });
            move || drop(listener)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::direction_for_key;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn arrow_keys_map_to_directions() {
        assert_eq!(direction_for_key("ArrowUp"), Some((0, -1)));
        assert_eq!(direction_for_key("ArrowDown"), Some((0, 1)));
        assert_eq!(direction_for_key("ArrowLeft"), Some((-1, 0)));
        assert_eq!(direction_for_key("ArrowRight"), Some((1, 0)));
    }

    #[wasm_bindgen_test]
    fn lowercase_wasd_map_to_directions() {
        assert_eq!(direction_for_key("w"), Some((0, -1)));
        assert_eq!(direction_for_key("a"), Some((-1, 0)));
        assert_eq!(direction_for_key("s"), Some((0, 1)));
        assert_eq!(direction_for_key("d"), Some((1, 0)));
    }

    #[wasm_bindgen_test]
    fn uppercase_wasd_map_to_directions() {
        assert_eq!(direction_for_key("W"), Some((0, -1)));
        assert_eq!(direction_for_key("A"), Some((-1, 0)));
        assert_eq!(direction_for_key("S"), Some((0, 1)));
        assert_eq!(direction_for_key("D"), Some((1, 0)));
    }

    #[wasm_bindgen_test]
    fn pause_keys_return_none() {
        // Pause is handled separately by the keyboard listener before the
        // direction lookup, so the pure mapper must NOT classify these as
        // direction inputs.
        assert_eq!(direction_for_key("Escape"), None);
        assert_eq!(direction_for_key("p"), None);
        assert_eq!(direction_for_key("P"), None);
    }

    #[wasm_bindgen_test]
    fn unrelated_keys_return_none() {
        assert_eq!(direction_for_key(" "), None);
        assert_eq!(direction_for_key("Enter"), None);
        assert_eq!(direction_for_key("Tab"), None);
        assert_eq!(direction_for_key(""), None);
        // Letters outside the WASD set (including accented/multibyte keys)
        // should fall through to `None`.
        assert_eq!(direction_for_key("x"), None);
        assert_eq!(direction_for_key("X"), None);
        assert_eq!(direction_for_key("q"), None);
        assert_eq!(direction_for_key("z"), None);
    }
}

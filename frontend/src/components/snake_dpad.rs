//! Touch-friendly four-button directional pad.
//!
//! Used on phones and tablets where a keyboard isn't available. Emits
//! `(dx, dy)` direction deltas — the same shape the keyboard handler
//! uses — so the rest of the game logic doesn't care which input device
//! produced the input.

use yew::prelude::*;

/// Props consumed by [`MobileDpad`].
#[derive(Properties, PartialEq)]
pub struct MobileDpadProps {
    /// Fires with the chosen direction delta in cell units.
    pub on_press: Callback<(i32, i32)>,
}

/// Four buttons (up / left / right / down) arranged around an empty
/// center cell.
#[function_component(MobileDpad)]
pub fn mobile_dpad(props: &MobileDpadProps) -> Html {
    let on_press = props.on_press.clone();
    html! {
        <div class="mobile-dpad">
            <div class="dpad-row">
                <button onclick={let on_press = on_press.clone(); Callback::from(move |_| on_press.emit((0, -1)))} class="dpad-btn up">{"▲"}</button>
            </div>
            <div class="dpad-row middle">
                <button onclick={let on_press = on_press.clone(); Callback::from(move |_| on_press.emit((-1, 0)))} class="dpad-btn left">{"◀"}</button>
                <div class="dpad-center"></div>
                <button onclick={let on_press = on_press.clone(); Callback::from(move |_| on_press.emit((1, 0)))} class="dpad-btn right">{"▶"}</button>
            </div>
            <div class="dpad-row">
                <button onclick={let on_press = on_press.clone(); Callback::from(move |_| on_press.emit((0, 1)))} class="dpad-btn down">{"▼"}</button>
            </div>
        </div>
    }
}

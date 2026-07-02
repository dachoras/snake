//! Modal overlay shown above the snake board.
//!
//! Renders one of four states, in priority order:
//! 1. Press-start screen (when the game hasn't started yet).
//! 2. Game-over screen with a name-entry form for the leaderboard.
//! 3. Pause screen with a resume button.
//! 4. Nothing (when the game is in progress).

use yew::prelude::*;

/// Props consumed by [`SnakeOverlay`].
#[derive(Properties, PartialEq)]
pub struct SnakeOverlayProps {
    /// `false` until the player presses the start button.
    pub started: bool,
    /// `true` after a wall or self collision.
    pub game_over: bool,
    /// `true` while the pause overlay is up.
    pub paused: bool,
    /// Current score, displayed on the game-over screen.
    pub score: u32,
    /// `true` while a leaderboard submission is in flight.
    pub submitting: bool,
    /// Player name typed into the game-over form.
    pub player_name: String,
    /// Fired when the player presses "PLAY AGAIN" or "PRESS START".
    pub on_restart: Callback<MouseEvent>,
    /// Fired when the game-over name form is submitted.
    pub on_submit_score: Callback<SubmitEvent>,
    /// Fired on every keystroke in the name field.
    pub on_name_input: Callback<InputEvent>,
    /// Fired when the player presses the resume button.
    pub on_resume: Callback<MouseEvent>,
}

/// Visual overlay layered above [`SnakeBoard`]. Routes the four game states
/// to their respective markup.
#[function_component(SnakeOverlay)]
pub fn snake_overlay(props: &SnakeOverlayProps) -> Html {
    let locale = use_context::<crate::i18n::LocaleContext>().expect("LocaleContext provided");

    if !props.started {
        html! {
            <div class="overlay start-overlay">
                <h2>{"SNAKE"}</h2>
                <button onclick={props.on_restart.clone()} class="btn-start">{locale.t("press_start")}</button>
            </div>
        }
    } else if props.game_over {
        html! {
            <div class="overlay gameover-overlay">
                <h2>{locale.t("game_over")}</h2>
                <p class="score-summary">{format!("{}: {}", locale.t("final_score"), props.score)}</p>

                <form onsubmit={props.on_submit_score.clone()} class="submit-score-form">
                    <input
                        type="text"
                        placeholder={locale.t("enter_name")}
                        value={props.player_name.clone()}
                        oninput={props.on_name_input.clone()}
                        class="name-input"
                        maxlength="15"
                        required=true
                    />
                    <button type="submit" class="btn-submit" disabled={props.submitting}>
                        {if props.submitting { locale.t("submitting") } else { locale.t("submit_score") }}
                    </button>
                </form>

                <button onclick={props.on_restart.clone()} class="btn-restart">{locale.t("play_again")}</button>
            </div>
        }
    } else if props.paused {
        html! {
            <div class="overlay pause-overlay">
                <h2>{locale.t("paused")}</h2>
                <button onclick={props.on_resume.clone()} class="btn-resume">{locale.t("resume")}</button>
            </div>
        }
    } else {
        html! {}
    }
}

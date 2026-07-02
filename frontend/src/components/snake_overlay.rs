use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SnakeOverlayProps {
    pub started: bool,
    pub game_over: bool,
    pub paused: bool,
    pub score: u32,
    pub submitting: bool,
    pub player_name: String,
    pub on_restart: Callback<MouseEvent>,
    pub on_submit_score: Callback<SubmitEvent>,
    pub on_name_input: Callback<InputEvent>,
    pub on_resume: Callback<MouseEvent>,
}

#[function_component(SnakeOverlay)]
pub fn snake_overlay(props: &SnakeOverlayProps) -> Html {
    let locale = use_context::<crate::i18n::LocaleContext>().unwrap();

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

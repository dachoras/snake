//! Top-3 leaderboard panel rendered next to the playfield.

use crate::api::LeaderboardEntry;
use yew::prelude::*;

/// Props consumed by [`LeaderboardPanel`].
#[derive(Properties, PartialEq)]
pub struct LeaderboardPanelProps {
    /// Full leaderboard as returned by the backend; only the top three
    /// entries are actually rendered.
    pub leaderboard: Vec<LeaderboardEntry>,
}

/// Renders the leaderboard header and the top three rows, or an empty-
/// state message when no scores exist yet.
#[function_component(LeaderboardPanel)]
pub fn leaderboard_panel(props: &LeaderboardPanelProps) -> Html {
    let locale = use_context::<crate::i18n::LocaleContext>().unwrap_or_default();
    html! {
        <div class="leaderboard-panel">
            <h3>{locale.t("leaderboard")}</h3>
            <div class="leaderboard-list">
                {
                    if props.leaderboard.is_empty() {
                        html! { <div class="leaderboard-empty">{locale.t("no_scores")}</div> }
                    } else {
                        html! {
                            <ul class="leaderboard-ol">
                                {
                                    for props.leaderboard.iter().take(3).enumerate().map(|(idx, entry)| {
                                        html! {
                                            <li key={idx} class="leaderboard-item">
                                                <span class="leader-name">{format!("{}. {}", idx + 1, entry.name)}</span>
                                                <span class="leader-score">{entry.score}</span>
                                            </li>
                                        }
                                    })
                                }
                            </ul>
                        }
                    }
                }
            </div>
        </div>
    }
}

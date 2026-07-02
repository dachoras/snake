use crate::api::LeaderboardEntry;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LeaderboardPanelProps {
    pub leaderboard: Vec<LeaderboardEntry>,
}

#[function_component(LeaderboardPanel)]
pub fn leaderboard_panel(props: &LeaderboardPanelProps) -> Html {
    let locale = use_context::<crate::i18n::LocaleContext>().unwrap();
    html! {
        <div class="leaderboard-panel">
            <h3>{locale.t("leaderboard")}</h3>
            <div class="leaderboard-list">
                {
                    if props.leaderboard.is_empty() {
                        html! { <div class="leaderboard-empty">{locale.t("no_scores")}</div> }
                    } else {
                        html! {
                            <ol class="leaderboard-ol">
                                {
                                    for props.leaderboard.iter().enumerate().map(|(idx, entry)| {
                                        html! {
                                            <li key={idx} class="leaderboard-item">
                                                <span class="leader-name">{&entry.name}</span>
                                                <span class="leader-score">{entry.score}</span>
                                            </li>
                                        }
                                    })
                                }
                            </ol>
                        }
                    }
                }
            </div>
        </div>
    }
}

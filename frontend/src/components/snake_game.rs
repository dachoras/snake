use crate::api::{ApiService, LeaderboardEntry};
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const GRID_SIZE: i32 = 20;

#[derive(Properties, PartialEq)]
pub struct SnakeGameProps {
    pub on_status: Callback<Option<(String, String)>>,
}

#[function_component(SnakeGame)]
pub fn snake_game(props: &SnakeGameProps) -> Html {
    let snake = use_state(|| vec![(10, 10), (10, 11), (10, 12)]);
    let direction = use_state(|| (0, -1)); // Up
    let next_direction = use_state(|| (0, -1));
    let food = use_state(|| (5, 5));
    let score = use_state(|| 0);
    let high_score = use_state(|| {
        if let Some(win) = web_sys::window()
            && let Ok(Some(storage)) = win.local_storage()
            && let Ok(Some(hs_val)) = storage.get_item("snake_high_score")
        {
            hs_val.parse::<u32>().unwrap_or(0)
        } else {
            0
        }
    });
    let game_over = use_state(|| false);
    let paused = use_state(|| false);
    let started = use_state(|| false);
    let leaderboard = use_state(|| Vec::<LeaderboardEntry>::new());
    let player_name = use_state(|| "".to_string());
    let submitting = use_state(|| false);

    let locale = use_context::<crate::i18n::LocaleContext>().unwrap();

    // Fetch leaderboard on load and game over
    {
        let leaderboard = leaderboard.clone();
        use_effect_with((), move |_| {
            let leaderboard = leaderboard.clone();
            spawn_local(async move {
                if let Ok(list) = ApiService::get_leaderboard().await {
                    leaderboard.set(list);
                }
            });
            || ()
        });
    }

    // Set notification status
    {
        let on_status = props.on_status.clone();
        let score_val = *score;
        let game_over_val = *game_over;
        let locale = locale.clone();
        use_effect_with((score_val, game_over_val), move |&(s, go)| {
            if go {
                on_status.emit(Some((
                    locale.t("game_over"),
                    "error".to_string(),
                )));
            } else {
                on_status.emit(Some((
                    format!("Score: {}", s),
                    "success".to_string(),
                )));
            }
            || ()
        });
    }

    // Random food generator helper
    let generate_food = {
        let snake = snake.clone();
        move || {
            let mut attempts = 0;
            loop {
                let x = (js_sys::Math::random() * GRID_SIZE as f64) as i32;
                let y = (js_sys::Math::random() * GRID_SIZE as f64) as i32;
                let on_snake = snake.iter().any(|&pos| pos == (x, y));
                if !on_snake || attempts > 100 {
                    return (x, y);
                }
                attempts += 1;
            }
        }
    };

    // Restart game function
    let on_restart = {
        let snake = snake.clone();
        let direction = direction.clone();
        let next_direction = next_direction.clone();
        let food = food.clone();
        let score = score.clone();
        let game_over = game_over.clone();
        let paused = paused.clone();
        let started = started.clone();
        let generate_food = generate_food.clone();
        Callback::from(move |_| {
            snake.set(vec![(10, 10), (10, 11), (10, 12)]);
            direction.set((0, -1));
            next_direction.set((0, -1));
            score.set(0);
            game_over.set(false);
            paused.set(false);
            started.set(true);
            food.set(generate_food());
        })
    };

    // Keyboard controls
    {
        let next_dir = next_direction.clone();
        let dir = direction.clone();
        let is_started = *started;
        let is_game_over = *game_over;
        let is_paused = *paused;
        let started = started.clone();
        let paused = paused.clone();
        let on_restart = on_restart.clone();

        use_effect_with((is_started, is_game_over, is_paused), move |&(st, go, ps)| {
            let listener = EventListener::new(&web_sys::window().unwrap(), "keydown", move |e| {
                let key_event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                let key = key_event.key();

                if go {
                    if key == "Enter" || key == " " {
                        on_restart.emit(MouseEvent::new("click").unwrap());
                    }
                    return;
                }

                if !st {
                    if key == "Enter" || key == " " || key.starts_with("Arrow") {
                        started.set(true);
                    }
                    return;
                }

                if key == "Escape" || key == "p" || key == "P" {
                    paused.set(!ps);
                    return;
                }

                if ps {
                    return;
                }

                let current_dir = *dir;
                let new_dir = match key.as_str() {
                    "ArrowUp" | "w" | "W" if current_dir.1 != 1 => Some((0, -1)),
                    "ArrowDown" | "s" | "S" if current_dir.1 != -1 => Some((0, 1)),
                    "ArrowLeft" | "a" | "A" if current_dir.0 != 1 => Some((-1, 0)),
                    "ArrowRight" | "d" | "D" if current_dir.0 != -1 => Some((1, 0)),
                    _ => None,
                };

                if let Some(nd) = new_dir {
                    key_event.prevent_default();
                    next_dir.set(nd);
                }
            });
            move || drop(listener)
        });
    }

    // Game loop tick
    {
        let snake = snake.clone();
        let dir = direction.clone();
        let next_dir = next_direction.clone();
        let food = food.clone();
        let score = score.clone();
        let high_score = high_score.clone();
        let game_over = game_over.clone();
        let is_started = *started;
        let is_paused = *paused;
        let is_game_over = *game_over;
        let generate_food = generate_food.clone();

        use_effect_with(
            (is_started, is_paused, is_game_over),
            move |&(st, ps, go)| {
                if !st || ps || go {
                    return Box::new(|| ()) as Box<dyn FnOnce()>;
                }

                let interval = Interval::new(150, move || {
                    let current_dir = *next_dir;
                    dir.set(current_dir);

                    let current_snake = (*snake).clone();
                    let head = current_snake[0];
                    let new_head = (head.0 + current_dir.0, head.1 + current_dir.1);

                    // Check wall collision
                    if new_head.0 < 0 || new_head.0 >= GRID_SIZE || new_head.1 < 0 || new_head.1 >= GRID_SIZE {
                        game_over.set(true);
                        return;
                    }

                    // Check self collision
                    if current_snake.iter().any(|&pos| pos == new_head) {
                        game_over.set(true);
                        return;
                    }

                    let mut next_snake = vec![new_head];
                    next_snake.extend_from_slice(&current_snake);

                    // Check food eating
                    if new_head == *food {
                        let new_score = *score + 10;
                        score.set(new_score);

                        if new_score > *high_score {
                            high_score.set(new_score);
                            if let Some(win) = web_sys::window()
                                && let Ok(Some(storage)) = win.local_storage()
                            {
                                let _ = storage.set_item("snake_high_score", &new_score.to_string());
                            }
                        }

                        food.set(generate_food());
                    } else {
                        next_snake.pop();
                    }

                    snake.set(next_snake);
                });

                Box::new(move || drop(interval))
            },
        );
    }

    // Submit leaderboard score
    let on_submit_score = {
        let name = player_name.clone();
        let score_val = *score;
        let submitting = submitting.clone();
        let leaderboard = leaderboard.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name_str = (*name).clone();
            if name_str.trim().is_empty() || *submitting {
                return;
            }
            submitting.set(true);
            let submitting = submitting.clone();
            let leaderboard = leaderboard.clone();
            spawn_local(async move {
                if ApiService::submit_score(&name_str, score_val).await.is_ok() {
                    if let Ok(list) = ApiService::get_leaderboard().await {
                        leaderboard.set(list);
                    }
                }
                submitting.set(false);
            });
        })
    };

    // On-screen D-Pad click handlers
    let set_next_dir = {
        let next_dir = next_direction.clone();
        let dir = direction.clone();
        move |dx: i32, dy: i32| {
            let current_dir = *dir;
            if (dx != 0 && current_dir.0 == 0) || (dy != 0 && current_dir.1 == 0) {
                next_dir.set((dx, dy));
            }
        }
    };

    html! {
        <div class="snake-container">
            <div class="game-wrapper">
                <div class="game-board-container">
                    <div class="score-board">
                        <div class="score-stat">
                            <span class="label">{"SCORE:"}</span>
                            <span class="value">{*score}</span>
                        </div>
                        <div class="score-stat">
                            <span class="label">{"HIGH:"}</span>
                            <span class="value">{*high_score}</span>
                        </div>
                    </div>

                    <div class="board-relative-wrapper">
                        <div class="game-grid">
                            {
                                for (0..GRID_SIZE).map(|y| {
                                    html! {
                                        <div class="grid-row" key={y}>
                                            {
                                                for (0..GRID_SIZE).map(|x| {
                                                    let is_snake_head = (*snake)[0] == (x, y);
                                                    let is_snake_body = !is_snake_head && snake.iter().any(|&pos| pos == (x, y));
                                                    let is_food = *food == (x, y);

                                                    let cell_class = if is_snake_head {
                                                        "grid-cell snake-head"
                                                    } else if is_snake_body {
                                                        "grid-cell snake-body"
                                                    } else if is_food {
                                                        "grid-cell food"
                                                    } else {
                                                        "grid-cell"
                                                    };

                                                    html! { <div class={cell_class} key={x}></div> }
                                                })
                                            }
                                        </div>
                                    }
                                })
                            }
                        </div>

                        {if !*started {
                            html! {
                                <div class="overlay start-overlay">
                                    <h2>{"SNAKE"}</h2>
                                    <button onclick={on_restart} class="btn-start">{"PRESS START"}</button>
                                </div>
                            }
                        } else if *game_over {
                            html! {
                                <div class="overlay gameover-overlay">
                                    <h2>{"GAME OVER"}</h2>
                                    <p class="score-summary">{format!("Final Score: {}", *score)}</p>

                                    <form onsubmit={on_submit_score} class="submit-score-form">
                                        <input
                                            type="text"
                                            placeholder="Enter your name"
                                            value={(*player_name).clone()}
                                            oninput={
                                                let player_name = player_name.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                                                    player_name.set(input.value());
                                                })
                                            }
                                            class="name-input"
                                            maxlength="15"
                                            required=true
                                        />
                                        <button type="submit" class="btn-submit" disabled={*submitting}>
                                            {if *submitting { "Submitting..." } else { "Submit Score" }}
                                        </button>
                                    </form>

                                    <button onclick={on_restart} class="btn-restart">{"PLAY AGAIN"}</button>
                                </div>
                            }
                        } else if *paused {
                            html! {
                                <div class="overlay pause-overlay">
                                    <h2>{"PAUSED"}</h2>
                                    <button onclick={
                                        let paused = paused.clone();
                                        Callback::from(move |_| paused.set(false))
                                    } class="btn-resume">{"RESUME"}</button>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>

                    // Visual controls for mobile/touch users
                    <div class="mobile-dpad">
                        <div class="dpad-row">
                            <button onclick={let set_dir = set_next_dir.clone(); Callback::from(move |_| set_dir(0, -1))} class="dpad-btn up">{"▲"}</button>
                        </div>
                        <div class="dpad-row middle">
                            <button onclick={let set_dir = set_next_dir.clone(); Callback::from(move |_| set_dir(-1, 0))} class="dpad-btn left">{"◀"}</button>
                            <div class="dpad-center"></div>
                            <button onclick={let set_dir = set_next_dir.clone(); Callback::from(move |_| set_dir(1, 0))} class="dpad-btn right">{"▶"}</button>
                        </div>
                        <div class="dpad-row">
                            <button onclick={let set_dir = set_next_dir.clone(); Callback::from(move |_| set_dir(0, 1))} class="dpad-btn down">{"▼"}</button>
                        </div>
                    </div>
                </div>

                // Sidebar leaderboard
                <div class="leaderboard-panel">
                    <h3>{"LEADERBOARD"}</h3>
                    <div class="leaderboard-list">
                        {
                            if leaderboard.is_empty() {
                                html! { <div class="leaderboard-empty">{"No high scores yet."}</div> }
                            } else {
                                html! {
                                    <ol class="leaderboard-ol">
                                        {
                                            for leaderboard.iter().enumerate().map(|(idx, entry)| {
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
            </div>
        </div>
    }
}

// Simple Helper EventListener wrapper for Yew
struct EventListener {
    target: web_sys::EventTarget,
    event_type: &'static str,
    closure: Option<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>>,
}

impl EventListener {
    fn new<F>(target: &web_sys::EventTarget, event_type: &'static str, mut callback: F) -> Self
    where
        F: FnMut(web_sys::Event) + 'static,
    {
        let closure = Closure::wrap(Box::new(move |e| callback(e)) as Box<dyn FnMut(web_sys::Event)>);
        target
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .unwrap();
        Self {
            target: target.clone(),
            event_type,
            closure: Some(closure),
        }
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            let _ = self.target.remove_event_listener_with_callback(
                self.event_type,
                closure.as_ref().unchecked_ref(),
            );
        }
    }
}

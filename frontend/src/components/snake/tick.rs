//! Interval loop that advances the snake one cell per tick.
//!
//! The tick speed is inversely proportional to the player's score: every
//! [`SCORE_PER_SPEEDUP`] points shave [`SPEEDUP_STEP_MS`] milliseconds off
//! the interval, clamped at [`MIN_TICK_MS`]. The effect re-runs (and
//! therefore resets the interval) whenever one of its dependencies
//! changes — start/pause/game-over/score.

use gloo_timers::callback::Interval;
use yew::prelude::*;

use super::super::snake_logic::{Pos, handle_tick};
use super::food::{GRID_SIZE, generate_food};

/// Tick interval floor. Even at high scores the snake never moves faster
/// than this rate.
const MIN_TICK_MS: u32 = 75;

/// Tick interval when the score is `0`. Acts as the starting speed.
const BASE_TICK_MS: u32 = 170;

/// Number of points required to shave one step off the tick interval.
const SCORE_PER_SPEEDUP: u32 = 20;

/// Milliseconds removed from the tick interval per [`SCORE_PER_SPEEDUP`]
/// points.
const SPEEDUP_STEP_MS: u32 = 15;

/// Bundles the [`UseStateHandle`]s consumed by the tick loop.
///
/// A single struct keeps the helper signature under
/// [`clippy::too_many_arguments`](../../../../clippy.toml) while still
/// allowing each handle to be cloned cheaply (they are reference-counted
/// internally).
#[derive(Clone)]
pub struct TickInputs {
    /// `true` once the player has pressed "PRESS START".
    pub started: UseStateHandle<bool>,
    /// `true` while the game is paused.
    pub paused: UseStateHandle<bool>,
    /// `true` after a collision; freezes the tick loop.
    pub game_over: UseStateHandle<bool>,
    /// Current score, drives tick speed.
    pub score: UseStateHandle<u32>,
    /// Last-applied direction (kept in sync with `next_direction` on tick).
    pub direction: UseStateHandle<Pos>,
    /// Player's most recent direction input; consumed at the next tick.
    pub next_direction: UseStateHandle<Pos>,
    /// Snake body cells, ordered head-first.
    pub snake: UseStateHandle<Vec<Pos>>,
    /// Current food position.
    pub food: UseStateHandle<Pos>,
    /// Persistent high score (mirrored to `localStorage`).
    pub high_score: UseStateHandle<u32>,
    /// `true` while the current food is the gold variant.
    pub is_gold: UseStateHandle<bool>,
}

/// Installs the recurring tick effect described in the module docs.
pub fn install_tick_loop(inputs: TickInputs) {
    let TickInputs {
        started,
        paused,
        game_over,
        score,
        direction,
        next_direction,
        snake,
        food,
        high_score,
        is_gold,
    } = inputs;

    let is_started = *started;
    let is_paused = *paused;
    let is_game_over = *game_over;
    let score_val = *score;
    let next_dir = next_direction.clone();
    let dir = direction.clone();
    let snake = snake.clone();
    let food = food.clone();
    let score = score.clone();
    let high_score = high_score.clone();
    let game_over = game_over.clone();
    let is_gold = is_gold.clone();

    use_effect_with(
        (is_started, is_paused, is_game_over, score_val),
        move |&(st, ps, go, s)| {
            if !st || ps || go {
                return Box::new(|| ()) as Box<dyn FnOnce()>;
            }
            let duration = std::cmp::max(
                MIN_TICK_MS,
                BASE_TICK_MS - (s / SCORE_PER_SPEEDUP) * SPEEDUP_STEP_MS,
            );
            let snake = snake.clone();
            let food = food.clone();
            let dir = dir.clone();
            let next_dir = next_dir.clone();
            let score = score.clone();
            let high_score = high_score.clone();
            let game_over = game_over.clone();
            let is_gold = is_gold.clone();
            let interval = Interval::new(duration, move || {
                handle_tick(
                    &snake,
                    &dir,
                    &next_dir,
                    &food,
                    &score,
                    &high_score,
                    &game_over,
                    &is_gold,
                    GRID_SIZE,
                    &|| generate_food(&snake),
                );
            });
            Box::new(move || drop(interval))
        },
    );
}

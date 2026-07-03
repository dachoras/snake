//! Tests for [`crate::components::snake_logic::apply_tick`].
//!
//! Kept in a sibling file so each crate source file stays under the 250-line
//! ceiling enforced by `.cursorrules`. The module is wired in only when
//! `cargo test` builds the crate, so it never bloats production builds.

use crate::components::snake_logic::{
    GOLD_FOOD_POINTS, Pos, PureTickInputs, REGULAR_FOOD_POINTS, apply_tick,
};
use wasm_bindgen_test::wasm_bindgen_test;

/// Builds a baseline [`PureTickInputs`] pointing the snake north with the
/// tail trailing south; tests tweak the fields they care about.
fn baseline() -> PureTickInputs {
    PureTickInputs {
        snake: vec![(10, 10), (10, 11), (10, 12)],
        direction: (0, -1),
        next_direction: (0, -1),
        food: (0, 0),
        score: 0,
        high_score: 0,
        game_over: false,
        is_gold: false,
        grid_size: 20,
    }
}

fn baseline_with_snake(snake: Vec<Pos>) -> PureTickInputs {
    PureTickInputs {
        snake,
        ..baseline()
    }
}

#[wasm_bindgen_test]
fn moves_head_in_next_direction_and_drops_tail() {
    let out = apply_tick(baseline(), 0.5, (5, 5));
    assert_eq!(out.direction, (0, -1));
    assert_eq!(out.snake, vec![(10, 9), (10, 10), (10, 11)]);
    assert_eq!(out.score, 0);
    assert_eq!(out.food, (0, 0));
    assert!(!out.game_over);
}

#[wasm_bindgen_test]
fn direction_advances_even_on_collision() {
    // Snake is at the top row moving up (-1 y), which is a wall collision.
    let mut input = baseline();
    input.snake = vec![(10, 0)];
    input.direction = (0, 1);
    input.next_direction = (0, -1);
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    assert_eq!(out.direction, (0, -1));
}

#[wasm_bindgen_test]
fn wall_collision_left_ends_game() {
    let mut input = baseline();
    input.snake = vec![(0, 5), (0, 6)];
    input.direction = (1, 0);
    input.next_direction = (-1, 0);
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    // Snake body untouched by the early-return.
    assert_eq!(out.snake, vec![(0, 5), (0, 6)]);
    assert_eq!(out.food, (0, 0));
    assert_eq!(out.score, 0);
}

#[wasm_bindgen_test]
fn wall_collision_right_ends_game() {
    let mut input = baseline();
    input.snake = vec![(19, 5), (19, 6)];
    input.next_direction = (1, 0);
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    assert_eq!(out.snake, vec![(19, 5), (19, 6)]);
}

#[wasm_bindgen_test]
fn wall_collision_top_ends_game() {
    let mut input = baseline();
    input.snake = vec![(5, 0), (5, 1)];
    input.next_direction = (0, -1);
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    assert_eq!(out.snake, vec![(5, 0), (5, 1)]);
}

#[wasm_bindgen_test]
fn wall_collision_bottom_ends_game() {
    let mut input = baseline();
    input.snake = vec![(5, 19), (5, 18)];
    input.next_direction = (0, 1);
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    assert_eq!(out.snake, vec![(5, 19), (5, 18)]);
}

#[wasm_bindgen_test]
fn self_collision_ends_game_without_scoring() {
    // Make a U-shape: snake body wraps around the head's next step.
    let mut input = baseline();
    input.snake = vec![(10, 10), (10, 9), (9, 9), (9, 10), (9, 11), (10, 11)];
    input.direction = (0, -1);
    input.next_direction = (0, -1);
    let expected_body = input.snake.clone();
    let out = apply_tick(input, 0.5, (5, 5));
    assert!(out.game_over);
    assert_eq!(out.snake, expected_body);
    assert_eq!(out.score, 0);
}

#[wasm_bindgen_test]
fn eating_food_grows_snake_and_awards_regular_points() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.next_direction = (0, -1);
    input.food = (5, 4);
    let out = apply_tick(input, 0.99, (15, 15));
    // Head moves to (5, 4); tail (5, 6) preserved -> length 3.
    assert_eq!(out.snake, vec![(5, 4), (5, 5), (5, 6)]);
    assert_eq!(out.score, REGULAR_FOOD_POINTS);
    assert_eq!(out.food, (15, 15));
    assert!(!out.is_gold);
}

#[wasm_bindgen_test]
fn eating_gold_food_awards_gold_points() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.next_direction = (0, -1);
    input.food = (5, 4);
    input.is_gold = true;
    let out = apply_tick(input, 0.5, (15, 15));
    assert_eq!(out.score, GOLD_FOOD_POINTS);
    assert_eq!(out.snake.len(), 3);
}

#[wasm_bindgen_test]
fn gold_roll_below_threshold_sets_next_food_gold() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.food = (5, 4);
    input.next_direction = (0, -1);
    // Below the 0.15 threshold -> next food should be gold.
    let out = apply_tick(input, 0.10, (15, 15));
    assert!(out.is_gold);
}

#[wasm_bindgen_test]
fn gold_roll_above_threshold_keeps_food_regular() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.food = (5, 4);
    input.next_direction = (0, -1);
    let out = apply_tick(input, 0.50, (15, 15));
    assert!(!out.is_gold);
}

#[wasm_bindgen_test]
fn gold_roll_unused_when_no_food_eaten() {
    // Head advances to (10, 9), which is not the food. is_gold should stay
    // whatever it was on entry.
    let out = apply_tick(baseline(), 0.0, (15, 15));
    assert_eq!(out.food, (0, 0));
    assert!(!out.is_gold);
}

#[wasm_bindgen_test]
fn high_score_lifts_when_score_passes_it() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.food = (5, 4);
    input.next_direction = (0, -1);
    input.high_score = 5;
    let out = apply_tick(input, 0.5, (15, 15));
    // 0 + 10 = 10, beats old high of 5 -> new high is 10.
    assert_eq!(out.score, 10);
    assert_eq!(out.high_score, 10);
}

#[wasm_bindgen_test]
fn high_score_stays_when_score_below_it() {
    let mut input = baseline_with_snake(vec![(5, 5), (5, 6)]);
    input.food = (5, 4);
    input.next_direction = (0, -1);
    input.score = 50;
    input.high_score = 100;
    let out = apply_tick(input, 0.5, (15, 15));
    assert_eq!(out.score, 60);
    assert_eq!(out.high_score, 100);
}

#[wasm_bindgen_test]
fn no_change_to_snake_when_grid_too_small() {
    // Snake head is at x=0; the move would land at x=-1 (off-grid).
    // The early-return path preserves the snake body as-is.
    let mut input = baseline_with_snake(vec![(0, 5), (0, 6)]);
    input.grid_size = 1;
    input.next_direction = (-1, 0);
    let out = apply_tick(input, 0.5, (15, 15));
    assert!(out.game_over);
    assert_eq!(out.snake, vec![(0, 5), (0, 6)]);
}

#[wasm_bindgen_test]
fn no_food_no_growth_just_translate() {
    let out = apply_tick(baseline(), 0.5, (1, 1));
    assert_eq!(out.snake.len(), 3);
    assert_eq!(out.snake[0], (10, 9));
}

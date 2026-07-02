//! English (default-fallback) translation table for the Snake UI.
//!
//! Only keys actually consumed by the UI live here; everything else was
//! inherited from a sibling app and is intentionally omitted to keep the
//! table small and to make missing translations obvious.

/// Look up an English string by its [`key`].
///
/// Returns `None` so the dispatcher in [`crate::i18n::translate`] can fall
/// back to the raw key when no locale matches.
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("SCORE"),
        "high_score" => Some("HIGH"),
        "press_start" => Some("PRESS START"),
        "game_over" => Some("GAME OVER"),
        "final_score" => Some("Final Score"),
        "enter_name" => Some("Enter your name"),
        "submitting" => Some("Submitting..."),
        "submit_score" => Some("Submit Score"),
        "play_again" => Some("PLAY AGAIN"),
        "paused" => Some("PAUSED"),
        "resume" => Some("RESUME"),
        "leaderboard" => Some("LEADERBOARD"),
        "no_scores" => Some("No high scores yet."),
        "login_locked" => Some("Locked Out"),
        "login_prompt" => Some("Enter PIN"),
        _ => None,
    }
}

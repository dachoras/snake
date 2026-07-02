//! German translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// German counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("PUNKTE"),
        "high_score" => Some("REKORD"),
        "press_start" => Some("START DRÜCKEN"),
        "game_over" => Some("SPIEL VORBEI"),
        "final_score" => Some("Endstand"),
        "enter_name" => Some("Name eingeben"),
        "submitting" => Some("Sende..."),
        "submit_score" => Some("Punkte senden"),
        "play_again" => Some("NOCHMAL SPIELEN"),
        "paused" => Some("PAUSE"),
        "resume" => Some("FORTSETZEN"),
        "leaderboard" => Some("BESTENLISTE"),
        "no_scores" => Some("Noch keine Einträge."),
        "login_locked" => Some("Gesperrt. Versuchen Sie es in 15 Minuten erneut."),
        "login_prompt" => Some("PIN zur Authentifizierung eingeben"),
        _ => None,
    }
}

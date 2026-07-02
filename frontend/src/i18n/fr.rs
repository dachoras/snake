//! French translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// French counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("SCORE"),
        "high_score" => Some("MEILLEUR"),
        "press_start" => Some("APPUYER SUR START"),
        "game_over" => Some("PARTIE TERMINÉE"),
        "final_score" => Some("Score Final"),
        "enter_name" => Some("Entrez votre nom"),
        "submitting" => Some("Envoi..."),
        "submit_score" => Some("Envoyer le Score"),
        "play_again" => Some("REJOUER"),
        "paused" => Some("PAUSE"),
        "resume" => Some("REPRENDRE"),
        "leaderboard" => Some("CLASSEMENT"),
        "no_scores" => Some("Aucun score pour l'instant."),
        "login_locked" => Some("Bloqué. Réessayez dans 15 minutes."),
        "login_prompt" => Some("Entrez le code PIN d'authentification"),
        _ => None,
    }
}

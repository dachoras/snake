//! Spanish translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// Spanish counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("PUNTUACIÓN"),
        "high_score" => Some("MÁXIMA"),
        "press_start" => Some("PULSAR START"),
        "game_over" => Some("FIN DE JUEGO"),
        "final_score" => Some("Puntuación Final"),
        "enter_name" => Some("Introduce tu nombre"),
        "submitting" => Some("Enviando..."),
        "submit_score" => Some("Enviar Puntuación"),
        "play_again" => Some("JUGAR OTRA VEZ"),
        "paused" => Some("PAUSADO"),
        "resume" => Some("REANUDAR"),
        "leaderboard" => Some("CLASIFICACIÓN"),
        "no_scores" => Some("Sin puntuaciones aún."),
        "login_locked" => Some("Bloqueado. Vuelve a intentarlo en 15 minutes."),
        "login_prompt" => Some("Introduce el PIN de autenticación para acceder"),
        _ => None,
    }
}

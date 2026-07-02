//! Portuguese translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// Portuguese counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("PONTUAÇÃO"),
        "high_score" => Some("RECORD"),
        "press_start" => Some("APERTE START"),
        "game_over" => Some("FIM DE JOGO"),
        "final_score" => Some("Pontuação Final"),
        "enter_name" => Some("Digite seu nome"),
        "submitting" => Some("Enviando..."),
        "submit_score" => Some("Enviar Pontuação"),
        "play_again" => Some("JOGAR NOVAMENTE"),
        "paused" => Some("PAUSADO"),
        "resume" => Some("RETOMAR"),
        "leaderboard" => Some("CLASSIFICAÇÃO"),
        "no_scores" => Some("Nenhuma pontuação ainda."),
        "login_locked" => Some("Bloqueado. Tente novamente em 15 minutos."),
        "login_prompt" => Some("Digite o PIN de autenticação para acessar"),
        _ => None,
    }
}

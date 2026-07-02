//! Russian translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// Russian counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("СЧЕТ"),
        "high_score" => Some("РЕКОРД"),
        "press_start" => Some("НАЖМИТЕ СТАРТ"),
        "game_over" => Some("ИГРА ОКОНЧЕНА"),
        "final_score" => Some("Итоговый счет"),
        "enter_name" => Some("Введите ваше имя"),
        "submitting" => Some("Отправка..."),
        "submit_score" => Some("Отправить рекорд"),
        "play_again" => Some("ИГРАТЬ СНОВА"),
        "paused" => Some("ПАУЗА"),
        "resume" => Some("ПРОДОЛЖИТЬ"),
        "leaderboard" => Some("ТАБЛИЦА РЕКОРДОВ"),
        "no_scores" => Some("Рекордов пока нет."),
        "login_locked" => Some("Заблокировано. Попробуйте через 15 минут."),
        "login_prompt" => Some("Введите PIN-код для доступа"),
        _ => None,
    }
}

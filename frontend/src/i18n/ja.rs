//! Japanese translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// Japanese counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("スコア"),
        "high_score" => Some("ハイスコア"),
        "press_start" => Some("スタートを押す"),
        "game_over" => Some("ゲームオーバー"),
        "final_score" => Some("最終スコア"),
        "enter_name" => Some("名前を入力してください"),
        "submitting" => Some("送信中..."),
        "submit_score" => Some("スコアを送信"),
        "play_again" => Some("もう一度遊ぶ"),
        "paused" => Some("一時停止"),
        "resume" => Some("再開"),
        "leaderboard" => Some("リーダーボード"),
        "no_scores" => Some("まだスコアがありません。"),
        "login_locked" => Some("ロックされました。15分後に再試行してください。"),
        "login_prompt" => Some("認証用のPINを入力してください"),
        _ => None,
    }
}

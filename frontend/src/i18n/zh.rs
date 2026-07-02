//! Simplified Chinese translation table for the Snake UI.
//!
//! Mirror of [`crate::i18n::en::translate`]. Missing keys fall through to
//! the English table at lookup time so a partial translation still
//! renders something useful. Add new entries here when a new key is
//! added to the English table.

/// Chinese counterpart of [`crate::i18n::en::translate`].
pub fn translate(key: &str) -> Option<&'static str> {
    match key {
        "score" => Some("得分"),
        "high_score" => Some("最高分"),
        "press_start" => Some("按下开始"),
        "game_over" => Some("游戏结束"),
        "final_score" => Some("最终得分"),
        "enter_name" => Some("输入名字"),
        "submitting" => Some("正在提交..."),
        "submit_score" => Some("提交得分"),
        "play_again" => Some("再玩一次"),
        "paused" => Some("暂停"),
        "resume" => Some("继续"),
        "leaderboard" => Some("排行榜"),
        "no_scores" => Some("暂无记录。"),
        "login_locked" => Some("已锁定。请15分钟后重试。"),
        "login_prompt" => Some("输入身份验证密码以访问"),
        _ => None,
    }
}

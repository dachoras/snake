use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::state::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
    pub date: String,
}

pub async fn get_leaderboard(State(state): State<AppState>) -> impl IntoResponse {
    let path = state.data_dir.join("leaderboard.json");
    let list: Vec<LeaderboardEntry> = match fs::read_to_string(&path).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    Json(list)
}

pub async fn submit_score(
    State(state): State<AppState>,
    Json(mut entry): Json<LeaderboardEntry>,
) -> impl IntoResponse {
    let path = state.data_dir.join("leaderboard.json");
    let mut list: Vec<LeaderboardEntry> = match fs::read_to_string(&path).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    };

    let now = time::OffsetDateTime::now_utc();
    let format = time::format_description::well_known::Rfc3339;
    entry.date = now.format(&format).unwrap_or_else(|_| "2026-07-02T00:00:00Z".to_string());

    // Sanitize name
    let name = entry.name.trim();
    entry.name = if name.is_empty() {
        "Anonymous".to_string()
    } else if name.len() > 15 {
        name[..15].to_string()
    } else {
        name.to_string()
    };

    list.push(entry);
    list.sort_by(|a, b| b.score.cmp(&a.score));
    list.truncate(10);

    if let Ok(content) = serde_json::to_string_pretty(&list) {
        let _ = fs::create_dir_all(&state.data_dir).await;
        let _ = fs::write(&path, content).await;
    }

    Json(list)
}

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::sync::RwLock;

pub use crate::config::AppConfig;

pub struct AppStateInner {
    // Config
    pub config: AppConfig,
    pub data_dir: PathBuf,
    pub leaderboard_file: PathBuf,

    // Active session IDs cache (random tokens, never the PIN).
    pub active_sessions: RwLock<std::collections::HashSet<String>>,

    // Per-IP request budget for general rate limiting (separate from PIN
    // brute-force lockouts, which now live in `shared_backend::auth::attempts`
    // and are global to the process).
    pub rate_limiter: RwLock<HashMap<IpAddr, Vec<Instant>>>,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub async fn ensure_data_dir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.data_dir).await?;

        if fs::metadata(&self.leaderboard_file).await.is_err() {
            println!("Initializing empty leaderboard.json");
            fs::write(&self.leaderboard_file, "[]").await?;
        }

        Ok(())
    }

    pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let max_requests = 100; // 100 requests
        let window = Duration::from_secs(60); // per 60 seconds
        let now = Instant::now();

        let mut map = self.rate_limiter.write().await;
        let timestamps = map.entry(ip).or_insert_with(Vec::new);

        timestamps.retain(|&t| now.duration_since(t) < window);

        if timestamps.len() >= max_requests {
            false
        } else {
            timestamps.push(now);
            true
        }
    }

    pub async fn clean_old_rate_limits(&self) {
        let window = Duration::from_secs(60);
        let now = Instant::now();
        let mut map = self.rate_limiter.write().await;
        map.retain(|_, timestamps| {
            timestamps.retain(|&t| now.duration_since(t) < window);
            !timestamps.is_empty()
        });
    }
}

//! Per-IP request-budget rate limiter.
//!
//! Distinct from the PIN-attempt lockout in `shared_backend::auth::attempts`:
//! that one throttles *failed* login attempts globally; this one throttles
//! the overall request rate per client IP. 100 requests per rolling 60-second
//! window per IP.

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

/// Default request budget: 100 requests.
pub const DEFAULT_MAX_REQUESTS: usize = 100;
/// Default window length: 60 seconds.
pub const DEFAULT_WINDOW: Duration = Duration::from_secs(60);

/// In-memory per-IP sliding-window counter map.
#[derive(Debug, Default)]
pub struct RateLimiter {
    inner: HashMap<IpAddr, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    /// Construct a limiter with the default budget and window.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            max_requests: DEFAULT_MAX_REQUESTS,
            window: DEFAULT_WINDOW,
        }
    }

    /// Construct a limiter with a custom budget and window (used in tests).
    #[must_use]
    pub fn with_limits(max_requests: usize, window: Duration) -> Self {
        Self {
            inner: HashMap::new(),
            max_requests,
            window,
        }
    }

    /// Record a hit from `ip` and return `true` if the request should be
    /// allowed (under budget), or `false` if the budget is exhausted.
    pub fn check(&mut self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let timestamps = self.inner.entry(ip).or_default();
        timestamps.retain(|&t| now.duration_since(t) < self.window);

        if timestamps.len() >= self.max_requests {
            false
        } else {
            timestamps.push(now);
            true
        }
    }

    /// Drop entries that no longer hold any timestamps inside the window.
    /// Called periodically from a background task.
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.inner.retain(|_, timestamps| {
            timestamps.retain(|&t| now.duration_since(t) < self.window);
            !timestamps.is_empty()
        });
    }

    /// Number of distinct IPs currently being tracked. Useful for tests and
    /// `/health` telemetry.
    #[must_use]
    pub fn tracked_ips(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn allows_under_budget() {
        let mut rl = RateLimiter::with_limits(3, Duration::from_secs(60));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
    }

    #[test]
    fn rejects_over_budget() {
        let mut rl = RateLimiter::with_limits(2, Duration::from_secs(60));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn budget_is_per_ip() {
        let mut rl = RateLimiter::with_limits(1, Duration::from_secs(60));
        let a = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 3));
        let b = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 4));
        assert!(rl.check(a));
        assert!(!rl.check(a));
        assert!(rl.check(b));
    }

    #[test]
    fn cleanup_drops_idle_ips() {
        let mut rl = RateLimiter::with_limits(5, Duration::from_millis(10));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 5));
        assert!(rl.check(ip));
        assert_eq!(rl.tracked_ips(), 1);
        std::thread::sleep(Duration::from_millis(20));
        rl.cleanup();
        assert_eq!(rl.tracked_ips(), 0);
    }

    #[test]
    fn expired_window_replenishes_budget() {
        let mut rl = RateLimiter::with_limits(1, Duration::from_millis(5));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 6));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
        std::thread::sleep(Duration::from_millis(10));
        assert!(rl.check(ip));
    }
}

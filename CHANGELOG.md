# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.27] - 2026-07-02

### Added
- Backend integration tests covering rate limiting, authentication, leaderboard persistence, and health check.
- Frontend tests for game-logic helpers (`generate_food`, `direction_for_key`, `apply_tick`).
- Optional `cargo deny` step in CI for supply-chain auditing.
- App version is now sourced from `/api/config` instead of being hardcoded.

### Changed
- `cookie` name renamed `PAD_PIN` → `SNAKE_PIN`.
- Date timestamps in leaderboard now produced by `chrono` (RFC 3339, infallible).
- Session IDs generated via `rand::rngs::OsRng` instead of a `/dev/urandom`-with-time-seeded-SHA-256 fallback.
- Frontend split into focused modules under `components/snake/` (state, food, tick, actions, keys) to enforce the 250-line file-size rule.
- `frontend/dist/Assets/manifest.json` PWA metadata corrected from the upstream "Log" notepad values to Snake's branding.

### Fixed
- UTF-8 byte slice panic in leaderboard name sanitizer (now truncates by `chars()`).
- Hardcoded fallback date replaced with proper error propagation.
- `Path::parent().unwrap()` (panic at filesystem root) replaced with explicit `web_root` resolution at startup.
- `/api/logout` cookie age built via unclamped `try_into` (could panic) replaced with clamped builder.
- Pre-existing typo in `frontend/Cargo.toml` (missing `}` on `web-sys` inline table) — surfaced by a clean Trunk build.

### Removed
- ~150 lines of dead "notepad" code paths carried over from the upstream shared fork.
- Tracked 2.1 GB vendored `frontend/Assets/shared-assets/shared-rust/` nested Cargo workspace.
- Unused backend dependencies: `notify`, `futures-util`.
- Stale `data/notepads.json` and `data/default.txt` artifacts.
- Stale `frontend/Assets/asset-manifest.json` referencing non-existent files.

### Security
- `redirect` URL sanitizer rejects `%2F`, `%5c`, double-encoded forms, control characters, and scheme-relative URLs in addition to the existing checks.
- Cookie maximum age clamped to `[1 minute, 30 days]` so a misconfigured `cookie_max_age_hours` cannot pin a session forever or zero it out.
- Hardcoded version strings removed from the frontend (was leaking through to JS until the `/api/config` response arrived).

## [1.0.26] - 2026-07-01

Carried over from upstream fork (pad/notepad). Pre-refactor state.

## [1.0.25]

Initial release under the UberMetroid organisation.
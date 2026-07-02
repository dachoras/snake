//! Yew components used by the Snake UI.
//!
//! Grouped by purpose:
//!
//! - [`header`] / [`footer`] — re-exports of the shared header and footer
//!   components that live in `shared-frontend`. Kept as local modules so
//!   consumers can `use crate::components::header::Header` without having
//!   to learn the upstream crate path.
//! - [`pin`] — PIN-gate login form.
//! - [`event_listener`] — RAII wrapper around `addEventListener` that
//!   removes the listener on drop.
//! - [`snake`] — the game itself: board, dpad, leaderboard, overlay,
//!   game logic, and the centralised state hook.
//! - [`snake_game`] — top-level game component that composes the snake
//!   sub-modules and renders the score / overlay layout.

pub mod event_listener;
pub mod footer;
pub mod header;
pub mod pin;
pub mod snake;
pub mod snake_board;
pub mod snake_dpad;
pub mod snake_game;
pub mod snake_leaderboard;
pub mod snake_logic;
pub mod snake_overlay;

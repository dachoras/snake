mod cli;
mod tui;
mod doctor;
mod data;
mod process;
mod status;

use std::env;

pub const APP_NAME: &str = "Snake";
pub const ENV_PREFIX: &str = "SNAKE";
pub const DB_FILE_NAME: &str = "leaderboard.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd = args[1].to_lowercase();
        if cmd == "tui" {
            tui::run_tui();
        } else {
            cli::handle_cli_args(&args);
        }
    } else {
        tui::run_tui();
    }
}

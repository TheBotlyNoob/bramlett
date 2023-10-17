#![warn(clippy::all)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::Application;

mod app;
mod rhai_fs_path;

fn main() -> iced::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    app::App::run(iced::Settings::default())
}

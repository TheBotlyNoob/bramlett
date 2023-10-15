#![warn(clippy::all, rust_2018_idioms)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;

mod app;
mod rhai_fs_path;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([350.0, 750.0].into()),
        min_window_size: Some([350.0, 750.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Bramlett's Totally Reliable Game Downloader",
        native_options,
        Box::new(|cc| Box::new(futures::executor::block_on(app::App::new(cc)).unwrap())),
    )
}

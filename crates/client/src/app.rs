use anyhow::Result;
use common::GameInfo;
use egui::RichText;
use std::path::PathBuf;
use bytes::Bytes;

#[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
const SERVER_URL: &str = "http://127.0.0.1:8000";
#[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
const SERVER_URL: &str = "https://bramletts-games.shuttleapp.rs";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {
    info: GameInfo,
    progress: f32,
    path: Option<PathBuf>,
    #[serde(skip)]
    promise: Option<poll_promise::Promise<Bytes>>
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    games: Vec<Game>,
}

impl App {
    /// Called once before the first frame.
    pub async fn new(_: &eframe::CreationContext<'_>) -> Result<Self> {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let games = reqwest::get(SERVER_URL)
            .await?
            .json::<Vec<GameInfo>>()
            .await?;

        Ok(Self {
            games: games
                .into_iter()
                .map(|info| Game {
                    info,
                    progress: 0.0,
                    path: None,
                    promise: None,
                })
                .collect(),
        })
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bramlett's Totally Reliable Game Downloader");
            ui.label("Click a game to download it. Open the downloaded file.");
            ui.label(
                RichText::new("The password for all files is \"game\" (without the quotes).")
                    .strong(),
            );

            ui.separator();
            ui.label("Due to time restraints, this app is not very polished. OMORI and Bloons will take a while to download.");
            ui.label("I recommend trying Geometry Dash, Papers Please, or FNAF for the time being.");
            ui.separator();

            for game in &self.games {
                ui.group(|ui| {
                    ui.label(&game.info.name);
                    // if game.promise.and_then(|p| p.ready()).is_none() {
                    //     ui.add(egui::ProgressBar::new(game.progress).desired_width(100.0));
                    // } else if game.progress != 1.0 {
                    //     if ui.button("Download").clicked() {
                    //         // let url = format!("{}/{}", SERVER_URL, game.info.path);
                    //         let promise = poll_promise::Promise::spawn_async(async move {
                    //         //     let mut response = reqwest::get(&url).await?;
                    //         //     let mut bytes = Bytes::new();
                    //         //     while let Some(chunk) = response.chunk().await? {
                    //         //         bytes.extend_from_slice(&chunk);
                    //         //     }
                    //         //     Ok(bytes)
                    //         });
                    //         // game.promise = Some(promise);
                    //     }
                    // } else {
                    //     ui.add(egui::Button::new("Run"));
                    // }
                    
                    // if let Some(path) = &game.path {
                    //     ui.hyperlink_to(path.to_string_lossy(), path);
                    // } else {
                    //     ui.add(egui::Button::new("Download").enabled(game.path.is_none()));
                    // }
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui)
            });
        });
    }
}

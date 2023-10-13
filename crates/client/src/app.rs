use anyhow::Result;
use common::GameInfo;
use egui::RichText;
use std::path::PathBuf;

#[cfg(debug_assertions)]
const SERVER_URL: &str = "http://127.0.0.1:8000";
#[cfg(not(debug_assertions))]
const SERVER_URL: &str = "https://bramlett-games.railway.app";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {
    info: GameInfo,
    progress: f32,
    path: Option<PathBuf>,
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

            for game in &mut self.games {
                ui.hyperlink_to(
                    &game.info.name,
                    format!(
                        "https://drive.google.com/uc?export=download&id={}",
                        game.info.gdrive_id
                    ),
                );
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui)
            });
        });
    }
}

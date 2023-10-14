use anyhow::Result;
use bytes::Bytes;
use common::GameInfo;
use egui::RichText;
use poll_promise::Promise;
use reqwest::{cookie::Jar, Client, ClientBuilder};
use std::{fmt::Debug, fs, io::Cursor, path::PathBuf, sync::Arc};
use tl::ParserOptions;
use zip::ZipArchive;

#[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
const SERVER_URL: &str = "http://127.0.0.1:8000";
#[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
const SERVER_URL: &str = "https://bramletts-games.shuttleapp.rs";

enum GameState {
    NotDownloaded,
    Downloading(Promise<Result<Bytes, reqwest::Error>>),
    Downloaded { dir: PathBuf },
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::NotDownloaded => f.debug_tuple("NotDownloaded").finish(),
            GameState::Downloading(..) => f.debug_tuple("Downloading").field(&"..").finish(),
            GameState::Downloaded { dir } => {
                f.debug_struct("Downloaded").field("dir", &dir).finish()
            }
        }
    }
}

pub struct Game {
    info: GameInfo,
    state: GameState,
}

pub struct App {
    games: Vec<Game>,
    client: Client,
}

impl App {
    /// Called once before the first frame.
    pub async fn new(_: &eframe::CreationContext<'_>) -> Result<Self> {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let client = ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(Arc::new(Jar::default()))
            .build()
            .unwrap();

        let games = client
            .get(SERVER_URL)
            .send()
            .await?
            .json::<Vec<GameInfo>>()
            .await?;

        Ok(Self {
            games: games
                .into_iter()
                .map(|info| Game {
                    info,
                    state: GameState::NotDownloaded,
                })
                .collect(),
            client,
        })
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bramlett's Totally Reliable Game Downloader");
            ui.label("Click a game to download it. Wait for it to download, then hit \"Run\".");
            ui.label("Some games may take a while to download. Please be patient.");

            for game in &mut self.games {
                ui.group(|ui| {
                    ui.label(&game.info.name);
                    match game.state {
                        GameState::NotDownloaded => {
                            if ui.button("Download").clicked() {
                                let url = format!(
                                    "https://drive.google.com/uc?export=download&id={}",
                                    game.info.gdrive_id
                                );
                                let promise = Promise::spawn_async({
                                    let client = self.client.clone();
                                    async move {
                                        // TODO: multithreaded download
                                        let response =
                                            client.get(&url).send().await?.text().await?;

                                        let real_url = {
                                            let dom =
                                                tl::parse(&response, ParserOptions::default())
                                                    .unwrap();
                                            let parser = dom.parser();
                                            dom.get_element_by_id("download-form")
                                                .unwrap()
                                                .get(parser)
                                                .unwrap()
                                                .as_tag()
                                                .unwrap()
                                                .attributes()
                                                .get("action")
                                                .unwrap()
                                                .unwrap()
                                                .as_utf8_str()
                                                .replace("&amp;", "&")
                                        };

                                        println!("real_url: {}", real_url);

                                        client.get(real_url).send().await?.bytes().await
                                    }
                                });
                                game.state = GameState::Downloading(promise);
                            }
                        }
                        GameState::Downloading(ref promise) => {
                            ui.label("Downloading... (this may take a while)");
                            if let Some(res) = promise.ready() {
                                match res {
                                    Ok(bytes) => {
                                        let dir = std::env::temp_dir().join(&game.info.name);
                                        std::fs::create_dir_all(&dir).unwrap();
                                        let mut archive =
                                            ZipArchive::new(Cursor::new(&bytes)).unwrap();

                                        for i in 0..archive.len() {
                                            let mut file = archive
                                                .by_index_decrypt(i, b"game")
                                                .unwrap()
                                                .unwrap();
                                            let mut filepath_components =
                                                file.enclosed_name().unwrap().components();
                                            filepath_components.next();

                                            let outpath = dir.join(filepath_components.as_path());

                                            if file.name().ends_with('/') {
                                                fs::create_dir_all(&outpath).unwrap();
                                            } else {
                                                if let Some(p) = outpath.parent() {
                                                    if !p.exists() {
                                                        fs::create_dir_all(p).unwrap();
                                                    }
                                                }
                                                let mut outfile =
                                                    fs::File::create(&outpath).unwrap();
                                                std::io::copy(&mut file, &mut outfile).unwrap();
                                            }
                                        }
                                        game.state = GameState::Downloaded { dir };
                                    }
                                    Err(err) => {
                                        ui.label(format!("Error: {}", err));
                                    }
                                }
                            };
                        }
                        GameState::Downloaded { ref dir } => {
                            if ui.button("Run").clicked() {
                                std::process::Command::new(dir.join(&game.info.exe))
                                    .current_dir(dir)
                                    .spawn()
                                    .unwrap();
                            }
                        }
                    }
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui)
            });
        });
    }
}

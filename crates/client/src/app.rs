use anyhow::{Context, Result};
use bytes::Bytes;
use common::GameInfo;
use egui::{RichText, Ui};
use poll_promise::Promise;
use reqwest::{cookie::Jar, Client, ClientBuilder};
use std::{cell::RefCell, fmt::Debug, fs, io::Cursor, path::PathBuf, rc::Rc, sync::Arc};
use tl::ParserOptions;
use zip::ZipArchive;

#[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
const SERVER_URL: &str = "http://127.0.0.1:8000";
#[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
const SERVER_URL: &str = "https://bramletts-games.shuttleapp.rs";

enum GameState {
    NotDownloaded,
    Downloading(Promise<Result<Bytes>>),
    Downloaded(Bytes, PathBuf),
    Installing(PathBuf, Promise<Result<()>>),
    Installed(PathBuf),
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::NotDownloaded => f.debug_tuple("NotDownloaded").finish(),
            GameState::Downloading(..) => f.debug_tuple("Downloading").field(&"..").finish(),
            GameState::Downloaded(bytes, dir) => f
                .debug_tuple("Downloaded")
                .field(&bytes)
                .field(&dir)
                .finish(),
            GameState::Installing(dir, ..) => f.debug_tuple("Installing").field(&dir).finish(),
            GameState::Installed(dir) => f.debug_tuple("Installing").field(&dir).finish(),
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
    error: Rc<RefCell<Option<String>>>,
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
            error: Rc::new(RefCell::new(None)),
        })
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(err) = &*self.error.borrow() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Error (ask me about it, I'll try to help over PH):");
                ui.separator();
                ui.label(RichText::new(err).strong());
            });
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bramlett's Totally Reliable Game Downloader");
            ui.label("Click a game to download it. Wait for it to download, then hit \"Run\".");
            ui.label("Some games may take a while to download. Please be patient.");

            let err = self.error.clone();
            for game in &mut self.games {
                ui.group(err_wrapper(err.clone(), |ui| {
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
                                                tl::parse(&response, ParserOptions::default())?;
                                            let parser = dom.parser();
                                            let bad_drive_ctx = "This really shouldn't happen. Google Drive did something weird with their downloading system.";
                                            dom.get_element_by_id("download-form")
                                                .context(bad_drive_ctx)?
                                                .get(parser)
                                                .context("shouldn't happen; .get(parser)")?
                                                .as_tag()
                                                .context("shouldn't happen; .as_tag()")?
                                                .attributes()
                                                .get("action")
                                                .context(bad_drive_ctx)?
                                                .context(bad_drive_ctx)?
                                                .as_utf8_str()
                                                .replace("&amp;", "&")
                                        };

                                        client
                                            .get(real_url)
                                            .send()
                                            .await?
                                            .bytes()
                                            .await
                                            .map_err(Into::into)
                                    }
                                });
                                game.state = GameState::Downloading(promise);
                            }
                        }
                        GameState::Downloading(ref promise) => {
                            if let Some(res) = promise.ready() {
                                let bytes =
                                    res.as_ref().map_err(|e| anyhow::anyhow!("{e:#?}"))?.clone();

                                game.state = GameState::Downloaded(
                                    bytes,
                                    std::env::temp_dir().join(&game.info.name),
                                );
                            } else {
                                ui.label("Downloading... (this may take a while)");
                            };
                        }
                        GameState::Downloaded(ref bytes, ref dir) => {
                            let dir = dir.clone();

                            let promise = Promise::<Result<()>>::spawn_blocking({
                                let dir = dir.clone();
                                let bytes = bytes.clone();

                                move || {
                                std::fs::create_dir_all(&dir)?;
                                let mut archive = ZipArchive::new(Cursor::new(&bytes))?;

                                for i in 0..archive.len() {
                                    let mut file = archive.by_index_decrypt(i, b"game")??;
                                    let mut filepath_components =
                                        file.enclosed_name().unwrap().components();
                                    filepath_components.next();

                                    let outpath = dir.join(filepath_components.as_path());

                                    if file.name().ends_with('/') {
                                        fs::create_dir_all(&outpath)?
                                    } else {
                                        if let Some(p) = outpath.parent() {
                                            if !p.exists() {
                                                fs::create_dir_all(p)?;
                                            }
                                        }
                                        let mut outfile = fs::File::create(&outpath)?;
                                        std::io::copy(&mut file, &mut outfile)?;
                                    }
                                    // make sure executable is executable on unix for wine users
                                    #[cfg(unix)]
                                    if let Some(ext) = outpath.extension() {
                                        if ext == "exe" {
                                            use std::os::unix::fs::PermissionsExt;
                                            let mut perms =
                                                fs::metadata(&outpath)?.permissions();
                                            perms.set_mode(0o755);
                                            fs::set_permissions(&outpath, perms)?;
                                        }
                                    }
                                }

                                Ok(())
                            }});

                            game.state = GameState::Installing(dir.clone(), promise);
                        }
                        GameState::Installing(ref dir, ref promise) => {
                            if let Some(res) = promise.ready() {
                                res.as_ref().map_err(|e| anyhow::anyhow!("{e:#?}"))?;

                                game.state = GameState::Installed(dir.clone());
                            } else {
                                ui.label("Installing... (this may also take a while)");
                            };
                        }
                        GameState::Installed(ref dir) => {
                            if ui.button("Run").clicked() {
                                std::process::Command::new(dir.join(&game.info.exe))
                                    .current_dir(dir)
                                    .spawn()?;
                            }
                        }
                    };

                    Ok(())
                }));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui)
            });
        });
    }
}

fn err_wrapper(
    err: Rc<RefCell<Option<String>>>,
    mut f: impl FnMut(&mut Ui) -> Result<()>,
) -> impl FnMut(&mut Ui) {
    move |ui| {
        if let Err(e) = f(ui) {
            err.borrow_mut().replace(format!("{e:#?}"));
        }
    }
}

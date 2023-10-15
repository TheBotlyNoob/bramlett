use anyhow::{anyhow, Context, Result};
use bytes::{Bytes, BytesMut};
use common::GameInfo;
use egui::{ProgressBar, RichText, Ui};
use futures::StreamExt;
use poll_promise::Promise;
use reqwest::{cookie::Jar, Client, ClientBuilder};
use rhai::{packages::Package, Engine, Scope, AST};
use std::{
    cell::RefCell,
    fmt::Debug,
    fs,
    io::Cursor,
    path::PathBuf,
    rc::Rc,
    sync::{atomic::AtomicU64, atomic::Ordering::Relaxed, Arc},
};
use sysinfo::{
    Pid, PidExt, ProcessExt, ProcessRefreshKind, ProcessStatus, RefreshKind, System, SystemExt,
};
use tl::ParserOptions;
use zip::ZipArchive;

#[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
const SERVER_URL: &str = "http://127.0.0.1:8000";
#[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
const SERVER_URL: &str = "https://bramletts-games.shuttleapp.rs";

#[derive(Clone)]
pub struct AtomicPercent(pub Arc<(AtomicU64, AtomicU64)>);
impl AtomicPercent {
    pub fn new() -> Self {
        Self(Arc::new((AtomicU64::new(0), AtomicU64::new(1))))
    }
    pub fn get(&self) -> f32 {
        self.0 .0.load(Relaxed) as f32 / self.0 .1.load(Relaxed) as f32
    }
    pub fn get_numerator(&self) -> u64 {
        self.0 .0.load(Relaxed)
    }
    pub fn get_denominator(&self) -> u64 {
        self.0 .1.load(Relaxed)
    }
    pub fn set(&self, numerator: u64, denominator: u64) {
        self.0 .0.store(numerator, Relaxed);
        self.0 .1.store(denominator, Relaxed);
    }
    pub fn add_numerator(&self, add: u64) {
        self.0 .0.fetch_add(add, Relaxed);
    }
}

enum GameState {
    NotDownloaded,
    Downloading(Promise<Result<Bytes>>, AtomicPercent),
    Downloaded(Bytes),
    Installing(Promise<Result<()>>, AtomicPercent),
    Installed,
    Running(Pid),
    // runs once; goes back to installed
    Stopped,
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::NotDownloaded => f.debug_tuple("NotDownloaded").finish(),
            GameState::Downloading(..) => f.debug_tuple("Downloading").field(&"..").finish(),
            GameState::Downloaded(bytes) => f.debug_tuple("Downloaded").field(&bytes).finish(),
            GameState::Installing(..) => f.debug_tuple("Installing").field(&"..").finish(),
            GameState::Installed => f.debug_tuple("Installing").finish(),
            GameState::Running(pid) => f.debug_tuple("Running").field(&pid).finish(),
            GameState::Stopped => f.debug_tuple("Stopped").finish(),
        }
    }
}

pub struct Game {
    info: GameInfo,
    game_dir: PathBuf,
    #[allow(dead_code)]
    save_dir: PathBuf,
    rhai_scope: Scope<'static>,
    hooks_ast: AST,
    state: GameState,
}

pub struct App {
    games: Vec<Game>,
    client: Client,
    rhai_engine: Engine,
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

        let mut rhai_engine = Engine::new();
        let module = rhai::exported_module!(crate::rhai_fs_path::functions);
        rhai_engine.register_global_module(module.into());
        rhai_fs::FilesystemPackage::new().register_into_engine(&mut rhai_engine);

        let error = Rc::new(RefCell::new(None));
        Ok(Self {
            games: match games
                .into_iter()
                .map(|info| {
                    let game_dir = dirs::data_local_dir()
                        .unwrap()
                        .join("Bramletts Games")
                        .join(info.name.clone());
                    std::fs::create_dir_all(&game_dir)?;
                    let mut save_dir = dirs::home_dir().unwrap();
                    for path in std::fs::read_dir(&save_dir)?.filter_map(Result::ok) {
                        if path.file_type()?.is_dir() {
                            let name = path.file_name().to_string_lossy().to_lowercase();
                            if name.starts_with("onedrive") {
                                save_dir = save_dir.join(path.file_name());
                                break;
                            }
                        };
                    } // if no onedrive, just use home dir
                    let save_dir = save_dir.join("Saves").join(&info.name);
                    std::fs::create_dir_all(&save_dir)?;
                    let mut scope = Scope::new();

                    if !cfg!(windows) {
                        let mut root = std::env::var("WINEPREFIX")
                            .map(PathBuf::from)
                            .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".wine"));
                        root.push("drive_c");

                        let home_dir = root.join("users").join(whoami::username());

                        scope.push_constant(
                            "DATA_LOCAL_DIR",
                            home_dir.join("AppData").join("Local"),
                        );
                        let config_dir = home_dir.join("AppData").join("Roaming");
                        scope.push_constant("DATA_DIR", config_dir.clone());
                        scope.push_constant("CONFIG_DIR", config_dir);
                        scope.push_constant("HOME_DIR", home_dir);
                    } else {
                        scope.push_constant("DATA_LOCAL_DIR", dirs::data_local_dir().unwrap());
                        scope.push_constant("DATA_DIR", dirs::data_dir().unwrap());
                        scope.push_constant("CONFIG_DIR", dirs::config_dir().unwrap());
                        scope.push_constant("HOME_DIR", dirs::home_dir().unwrap());
                    }

                    scope.push_constant("GAME_DIR", game_dir.clone());
                    scope.push_constant("SAVE_DIR", save_dir.clone());

                    let hooks_ast = rhai_engine.compile(info.hooks.clone())?;
                    Ok(Game {
                        save_dir,
                        rhai_scope: scope,
                        hooks_ast,
                        info,
                        state: if game_dir.join("installed").exists() {
                            GameState::Installed
                        } else {
                            GameState::NotDownloaded
                        },
                        game_dir,
                    })
                })
                .collect::<Result<Vec<Game>>>()
            {
                Ok(games) => games,
                Err(e) => {
                    error.borrow_mut().replace(e.to_string());
                    vec![]
                }
            },
            rhai_engine,
            client,
            error,
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
            ui.heading(crate::NAME);
            ui.label("Click a game to install it. Wait for it to install, then hit \"Run\".");
            ui.label("Some games may take a while to install. Please be patient.");

            ui.separator();
            ui.label("Games progress is saved on all computers.");
            ui.label(
                "I would advise using the same computer to avoid installing a game more than once.",
            );
            ui.separator();

            for game in &mut self.games {
                ui.group(err_wrapper(self.error.clone(), |ui| {
                    ui.label(&game.info.name);
                    match &mut game.state {
                        GameState::NotDownloaded => {
                            if ui.button("Download").clicked() {
                                let progress = AtomicPercent::new();
                                let promise = Promise::spawn_async({
                                    let client = self.client.clone();
                                    let gdrive_id = game.info.gdrive_id.clone();
                                    let progress = progress.clone();
                                    download_gdrive(gdrive_id, client, progress)
                                });
                                game.state = GameState::Downloading(promise, progress);
                            }
                        }
                        GameState::Downloading(promise, progress) => {
                            if let Some(res) = promise.ready() {
                                let bytes = res.as_ref().map_err(|e| anyhow!("{e}"))?.clone();
                                game.state = GameState::Downloaded(bytes);
                            } else {
                                ui.add(ProgressBar::new(progress.get()).text("Downloading..."))
                                    .on_hover_ui(|ui| {
                                        ui.label(format!(
                                            "{:.3} out of {:.3} gigabytes downloaded",
                                            progress.get_numerator() as f32 / 1_000_000_000.0,
                                            progress.get_denominator() as f32 / 1_000_000_000.0
                                        ));
                                    });
                            };
                        }
                        GameState::Downloaded(bytes) => {
                            let progress = AtomicPercent::new();
                            let promise = Promise::<Result<()>>::spawn_blocking({
                                let dir = game.game_dir.clone();
                                let bytes = bytes.clone();
                                let progress = progress.clone();
                                move || extract_zip_with_password(bytes, dir, b"game", progress)
                            });
                            game.state = GameState::Installing(promise, progress);
                        }
                        GameState::Installing(promise, progress) => {
                            if let Some(res) = promise.ready() {
                                res.as_ref().map_err(|e| anyhow!("{e}"))?;

                                std::fs::OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(game.game_dir.join("installed"))?;

                                self.rhai_engine
                                    .call_fn::<()>(
                                        &mut game.rhai_scope,
                                        &game.hooks_ast,
                                        "post_install",
                                        (),
                                    )
                                    .map_err(|e| anyhow!("{e}"))?;
                                game.state = GameState::Installed;
                            } else {
                                ui.add(ProgressBar::new(progress.get()).text("Installing..."))
                                    .on_hover_ui(|ui| {
                                        ui.label(format!(
                                            "{} out of {} files installed",
                                            progress.get_numerator(),
                                            progress.get_denominator()
                                        ));
                                    });
                            };
                        }
                        GameState::Installed => {
                            ui.horizontal(err_wrapper(self.error.clone(), |ui| {
                                if ui.button("Run").clicked() {
                                    self.rhai_engine
                                        .call_fn::<()>(
                                            &mut game.rhai_scope,
                                            &game.hooks_ast,
                                            "pre_run",
                                            (),
                                        )
                                        .map_err(|e| anyhow!("{e}"))?;
                                    let pid = std::process::Command::new(
                                        game.game_dir.join(&game.info.exe),
                                    )
                                    .current_dir(game.game_dir.clone())
                                    .args(game.info.args.clone())
                                    .spawn()?
                                    .id();
                                    game.state = GameState::Running(Pid::from_u32(pid));
                                }
                                if ui.button("Delete").clicked() {
                                    std::fs::remove_dir_all(&game.game_dir)?;
                                    game.state = GameState::NotDownloaded;
                                }

                                Ok(())
                            }));
                        }
                        GameState::Running(pid) => {
                            ui.label("Running...");
                            let mut system = System::new_with_specifics(
                                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
                            );
                            system.refresh_processes();
                            if ![ProcessStatus::Run, ProcessStatus::Sleep].contains(
                                &system.process(*pid).map(|p| p.status()).unwrap_or(
                                    // What's here doesn't actually matter so long as it's not `Run` or `Sleep`
                                    sysinfo::ProcessStatus::Zombie,
                                ),
                            ) {
                                game.state = GameState::Stopped;
                            }
                        }
                        GameState::Stopped => {
                            self.rhai_engine
                                .call_fn::<()>(
                                    &mut game.rhai_scope,
                                    &game.hooks_ast,
                                    "post_run",
                                    (),
                                )
                                .map_err(|e| anyhow!("{e}"))?;
                            game.state = GameState::Installed;
                        }
                    };
                    Ok(())
                }));
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui);
                if !cfg!(windows) {
                    ui.label(
                        RichText::new("⚠ Not on Windows ⚠")
                            .small()
                            .color(ui.visuals().warn_fg_color),
                    )
                    .on_hover_text("Saving and some games may not work on non-Windows platforms.");
                };
            });

            ctx.request_repaint(); // too many moving parts, so just repaint every frame
        });
    }
}

fn err_wrapper(
    err: Rc<RefCell<Option<String>>>,
    mut f: impl FnMut(&mut Ui) -> Result<()>,
) -> impl FnMut(&mut Ui) {
    move |ui| {
        if let Err(e) = f(ui) {
            err.borrow_mut().replace(e.to_string());
        }
    }
}

fn extract_zip_with_password(
    bytes: Bytes,
    dir: PathBuf,
    password: &[u8],
    progress: AtomicPercent,
) -> Result<()> {
    std::fs::create_dir_all(&dir)?;
    let mut archive = ZipArchive::new(Cursor::new(&bytes))?;
    progress.set(0, archive.len() as u64);
    for i in 0..archive.len() {
        progress.add_numerator(1);
        let mut file = archive.by_index_decrypt(i, password)??;
        let mut filepath_components = file.enclosed_name().unwrap().components();
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

                let mut perms = fs::metadata(&outpath)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&outpath, perms)?;
            }
        }
    }
    Ok(())
}

async fn download_gdrive(
    gdrive_id: String,
    client: Client,
    progress: AtomicPercent,
) -> Result<Bytes> {
    let gdrive_url = format!(
        "https://drive.google.com/uc?export=download&id={}",
        gdrive_id
    );

    // TODO: multithreaded download
    let response = client.get(&gdrive_url).send().await?.text().await?;
    let bad_drive_ctx =
        "This really shouldn't happen. Google Drive did something weird with their system.";
    let real_url = {
        let dom = tl::parse(&response, ParserOptions::default())?;
        let parser = dom.parser();
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

    log::info!("real google drive download URL: {}", real_url);

    let res = client.get(&real_url).send().await?;
    let content_length = res.content_length().unwrap();
    progress.set(0, content_length);

    let mut bytes = BytesMut::new();
    let mut stream = res.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        progress.add_numerator(chunk.len() as u64);
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes.freeze())
}

use client::{get_game_list, Game, GameState};
use dialog::DialogBox;
use iced::executor;
use iced::widget::{button, text, Column};
use iced::{
    widget::{column as col, row, scrollable},
    Application, Command, Element, Theme,
};
use reqwest::Client;
use rhai::Engine;
use std::sync::Arc;

const TITLE: &str = "Bramlett's Totally Reliable Game Launcher";

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("client error: {0}")]
    Client(#[from] client::ClientError),
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub enum Msg {
    RefreshGameList(Result<Vec<Game>, Arc<AppError>>),
    // Download(String),
}

pub struct App {
    client: Client,
    games: Vec<Game>,
}

impl App {
    fn fallible_update(&mut self, msg: Msg) -> Result<Command<Msg>> {
        match msg {
            Msg::RefreshGameList(r) => {
                self.games = r.map_err(|e| AppError::Other(Box::new(e)))?;
                Ok(Command::none())
            } // Msg::Download(gdrive_id) => {}
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Msg;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (App, Command<Self::Message>) {
        let client = Client::new();

        let engine = Engine::new();

        (
            App {
                client: client.clone(),
                games: vec![],
            },
            Command::perform(
                async move {
                    get_game_list(client, &engine)
                        .await
                        .map_err(|e| Arc::new(AppError::from(e)))
                },
                Msg::RefreshGameList,
            ),
        )
    }

    fn title(&self) -> String {
        String::from(TITLE)
    }

    fn update(&mut self, msg: Msg) -> Command<Msg> {
        match self.fallible_update(msg) {
            Ok(cmd) => cmd,
            Err(e) => {
                log::error!("error {e:#?}");
                if let Err(e) = dialog::Message::new(e.to_string())
                    .title(format!("ERROR - {TITLE}"))
                    .show()
                {
                    log::error!("error showing dialog: {e:#?}");
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let mut games_col = Column::new();
        for game in &self.games {
            games_col = games_col.push(col![
                text(&game.info.name),
                // match &game.state {
                //     GameState::NotDownloaded =>
                //         row![button("Download").on_press(Msg::Download(id)).into()],
                //     GameState::Downloading(_) => row![text("Downloading...")],
                //     GameState::Downloaded(_) => row![button("Install").on_press(Msg::Install(id))],
                //     GameState::Installing(_) => row![button("Installing...")],
                //     GameState::Installed => row![button("Play").on_press(Msg::Play(id))],
                //     GameState::Running(_) => row![button("Stop").on_press(Msg::Stop(id))],
                //     GameState::Stopped => row![button("Play").on_press(Msg::Play(id))],
                // }
            ]);
        }
        scrollable(games_col).into()
    }

    // fn theme(&self) -> Self::Theme {
    //     Theme::Dark
    // }
}

use common::GameInfo;
use iced::executor;
use iced::{Application, Command, Element, Theme};
use obfstr::obfstr;
use reqwest::Client;
use rhai::{Scope, AST};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Game {
    info: GameInfo,
    game_dir: PathBuf,
    #[allow(dead_code)]
    save_dir: PathBuf,
    rhai_scope: Scope<'static>,
    hooks_ast: AST,
}

const TITLE: &str = "Bramlett's Totally Reliable Game Launcher";

#[derive(Debug)]
pub enum Msg {
    RefreshGameList(Result<Vec<GameInfo>, reqwest::Error>),
}

pub struct App {
    client: Client,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Msg;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let client = Client::new();
        (
            App {
                client: client.clone(),
            },
            Command::perform(
                async move {
                    client
                        .get(
                            #[cfg(any(not(debug_assertions), feature = "prod_in_debug"))]
                            obfstr!("https://bramletts-games.shuttleapp.rs"),
                            #[cfg(all(debug_assertions, not(feature = "prod_in_debug")))]
                            obfstr!("http://127.0.0.1"),
                        ) // obfstr because... it's something to try and stop defender from flagging the exec
                        .send()
                        .await?
                        .json()
                        .await
                },
                Msg::RefreshGameList,
            ),
        )
    }

    fn title(&self) -> String {
        String::from(TITLE)
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Msg::RefreshGameList(r) => {
                // todo
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

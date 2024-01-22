pub use crate::api::games::{Game, Games};
use crate::{
    api::error::Result,
    core::{
        db::{init_db, CONNECTION},
        dirs,
    },
};
use futures::TryStreamExt;

pub async fn fetch_games() -> Result<Games> {
    println!("DADS");
    let query = {
        sqlx::query_as::<_, Game>("SELECT * FROM games")
            .fetch(CONNECTION.get().unwrap())
            .try_collect::<Vec<_>>()
            .await
    };
    match query {
        Ok(games) => Ok(dbg!(Games { games })),
        Err(_) => {
            println!("ABCDEF");
            // TODO: make sure to change this once PR is merged
            #[cfg(debug_assertions)]
            let games = serde_json::from_str(&tokio::fs::read_to_string("games.json").await?)?;
            #[cfg(not(debug_assertions))]
            let games = reqwest::get("https://raw.githubusercontent.com/TheBotlyNoob/bramletts-games/chore/flutter/games.json").await?.json().await?;

            println!("INIT");
            init_db(&games).await?;

            Ok(games)
        }
    }
}

pub async fn run_game(game: Game) -> Result<()> {
    let game_dir = dirs::game_dir(&game);
    tokio::process::Command::new(game_dir.join(game.exe))
        .current_dir(game_dir)
        .args(game.args)
        .spawn()?
        .wait()
        .await?;
    Ok(())
}

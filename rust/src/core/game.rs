pub use crate::api::games::{Game, Games};
use crate::{
    api::error::Result,
    core::{
        db::{init_db, update_db, CONNECTION},
        dirs,
    },
};
use futures::TryStreamExt;

pub async fn fetch_games() -> Result<Vec<Game>> {
    let query = {
        sqlx::query_as::<_, Game>("SELECT * FROM games")
            .fetch(CONNECTION.get().unwrap())
            .try_collect::<Vec<_>>()
            .await
    };

    let new_games = match get_new_games().await {
        Ok(g) => g.games,
        Err(e) => {
            log::error!("error getting new games: {e:#?}");
            Vec::new()
        }
    };

    let old_games = match query {
        Ok(games) => games,
        Err(_) => {
            init_db().await?;

            new_games.clone()
        }
    };

    update_db(
        old_games.clone().into_iter().collect(),
        new_games.clone().into_iter().collect(),
    )
    .await?;

    Ok(if new_games.is_empty() {
        old_games
    } else {
        new_games
    })
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

pub async fn get_new_games() -> Result<Games> {
    #[cfg(debug_assertions)]
    return Ok(serde_json::from_str(
        &tokio::fs::read_to_string("games.json").await?,
    )?);

    #[cfg(not(debug_assertions))]
    Ok(reqwest::get(
        "https://raw.githubusercontent.com/TheBotlyNoob/bramletts-games/master/games.json",
    )
    .await?
    .json()
    .await?)
}

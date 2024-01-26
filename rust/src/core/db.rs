use crate::api::error::Result;
use sqlx::{Executor, QueryBuilder, Sqlite, SqlitePool};
use std::{collections::HashSet, sync::OnceLock};

use super::game::Game;

pub static CONNECTION: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_conn() {
    #[cfg(debug_assertions)]
    let db_path = std::path::PathBuf::from("games.db");
    #[cfg(not(debug_assertions))]
    let db_path = crate::core::dirs::config_dir().join("games.db");

    let _ = tokio::fs::create_dir_all(db_path.parent().unwrap()).await;
    let _ = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&db_path)
        .await;

    let conn = SqlitePool::connect(&format!("sqlite:{}", db_path.display()))
        .await
        .unwrap();
    let _ = CONNECTION.set(conn);
}

// if we try to take references of Game in the HashSets, we get some bogus "higher-ranked lifetime error".
// https://github.com/rust-lang/rust/issues/102211 maybe?
pub async fn update_db(prev_games: HashSet<Game>, new_games: HashSet<Game>) -> Result<()> {
    let conn = CONNECTION.get().unwrap();

    let mut query_builder = QueryBuilder::<Sqlite>::new(
        "INSERT INTO games (name, exe, args, icon, url, uuid, sha256, state) ",
    );

    let mut diff = new_games
        .into_iter()
        .filter(|g| prev_games.contains(g))
        .peekable();

    if diff.peek().is_some() {
        query_builder.push_values(diff.take(999), |mut b, game| {
            b.push_bind(game.name)
                .push_bind(game.exe)
                .push_bind(serde_json::to_string(&game.args).unwrap())
                .push_bind(game.icon)
                .push_bind(game.url)
                .push_bind(game.uuid)
                .push_bind(game.sha256)
                .push_bind(game.state as u8);
        });

        let query = query_builder.build();

        query.execute(conn).await?;
    }

    Ok(())
}

pub async fn init_db() -> Result<()> {
    let conn = CONNECTION.get().unwrap();
    conn.execute("
CREATE TABLE games 
        (name VARCHAR(255), exe VARCHAR(50), args VARCHAR(255), icon VARCHAR(255), 
        url VARCHAR(150), uuid CHAR(36), sha256 CHAR(64), state BOOLEAN NOT NULL CHECK (state IN (0, 1)));
    ").await?;

    Ok(())
}

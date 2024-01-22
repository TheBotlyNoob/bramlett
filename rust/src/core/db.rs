use crate::{
    api::{error::Result, games::Games},
    core::dirs,
};
use sqlx::{Executor, SqlitePool};
use std::sync::OnceLock;

pub static CONNECTION: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_conn() {
    #[cfg(debug_assertions)]
    let db_path = std::path::PathBuf::from("games.db");
    #[cfg(not(debug_assertions))]
    let db_path = dirs::config_dir().join("games.db");

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

pub async fn init_db(games: &Games) -> Result<()> {
    let conn = CONNECTION.get().unwrap();
    conn.execute("
CREATE TABLE games 
        (name VARCHAR(255), exe VARCHAR(50), args VARCHAR(255), icon VARCHAR(255), 
        url VARCHAR(150), uuid CHAR(36), sha256 CHAR(64), state BOOLEAN NOT NULL CHECK (state IN (0, 1)));
    ").await?;

    for game in &games.games {
        conn.execute(
            sqlx::query("INSERT INTO games VALUES(?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&game.name)
                .bind(&game.exe)
                .bind(serde_json::to_string(&game.args)?)
                .bind(&game.icon)
                .bind(&game.url)
                .bind(&game.uuid)
                .bind(&game.sha256)
                .bind(game.state as u8),
        )
        .await?;
    }
    Ok(())
}

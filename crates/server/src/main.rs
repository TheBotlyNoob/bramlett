use axum::{routing::get, Json, Router};
use common::GameInfo;

fn games() -> Vec<GameInfo> {
    // the passwd for all is "game"
    vec![
        GameInfo {
            name: "Papers Please".into(),
            gdrive_id: "1HzLYxrQQ1a8wj89KwTXMQ_hwdE4lCLvr".into(),
        },
        GameInfo {
            name: "Geometry Dash".into(),
            gdrive_id: "1vUtZqg6eDpRlKWNzDkjwq9_REN2LfYHt".into(),
        },
        GameInfo {
            name: "Bloons TD 6".into(),
            gdrive_id: "16paRX1A9qxvFyWzgGGremMviKWvT0fuN".into(),
        },
        GameInfo {
            name: "OMORI".into(),
            gdrive_id: "1liGmClrNJu2yPmX3HdyMoO_QXI64gJKy".into(),
        },
        GameInfo {
            name: "Five Nights at Freddy's".into(),
            gdrive_id: "1gnn4X4OtIVuB-t6ZblBN_UUD4NawbzJf".into(),
        },
        GameInfo {
            name: "Five Nights at Freddy's 2".into(),
            gdrive_id: "1gPjMCiX5IHnhoXi18jGAbHf1CiMlzSEf".into(),
        },
    ]
}

#[shuttle_runtime::main]

async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { Json(games()) }))
        .into())
}

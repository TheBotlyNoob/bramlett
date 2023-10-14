use axum::{routing::get, Json, Router};
use common::GameInfo;
use std::path::PathBuf;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        // `GET /` goes to `root`
        .route(
            "/",
            get(|| async {
                Json([
                    GameInfo {
                        name: "Papers Please".into(),
                        gdrive_id: "1HzLYxrQQ1a8wj89KwTXMQ_hwdE4lCLvr".into(),
                        exe: PathBuf::from("Papers Please/PapersPlease.exe"),
                    },
                    GameInfo {
                        name: "Geometry Dash".into(),
                        gdrive_id: "1vUtZqg6eDpRlKWNzDkjwq9_REN2LfYHt".into(),
                        exe: PathBuf::from("Geometry Dash/GeometryDash.exe"),
                    },
                    GameInfo {
                        name: "Bloons TD 6".into(),
                        gdrive_id: "16paRX1A9qxvFyWzgGGremMviKWvT0fuN".into(),
                        exe: PathBuf::from("BloonsTD6.exe"),
                    },
                    GameInfo {
                        name: "OMORI".into(),
                        gdrive_id: "1liGmClrNJu2yPmX3HdyMoO_QXI64gJKy".into(),
                        exe: PathBuf::from("OMORI.exe"),
                    },
                    GameInfo {
                        name: "Five Nights at Freddy's".into(),
                        gdrive_id: "1gnn4X4OtIVuB-t6ZblBN_UUD4NawbzJf".into(),
                        exe: PathBuf::from("FiveNightsatFreddys.exe"),
                    },
                    GameInfo {
                        name: "Five Nights at Freddy's 2".into(),
                        gdrive_id: "1gPjMCiX5IHnhoXi18jGAbHf1CiMlzSEf".into(),
                        exe: PathBuf::from("FiveNightsatFreddys2.exe"),
                    },
                    GameInfo {
                        name: "Portal".into(),
                        gdrive_id: "14eIkhwzFvZ8T2jEVs_1loEv9A2TcZSP2".into(),
                        exe: PathBuf::from("Portal.exe"),
                    },
                ])
            }),
        )
        .into())
}

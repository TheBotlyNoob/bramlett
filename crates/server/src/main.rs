use axum::{routing::get, Json, Router};
use common::{GameId, GameInfo};

macro_rules! script {
    ($name: literal) => {
        include_str!(concat!("../scripts/", $name, ".rhai")).into()
    };
}

fn games() -> Vec<GameInfo> {
    vec![
        GameInfo {
            name: "Papers Please".into(),
            id: GameId(0),
            gdrive_id: "1HzLYxrQQ1a8wj89KwTXMQ_hwdE4lCLvr".into(),
            exe: "PapersPlease.exe".into(),
            hooks: script!("papers_please"),
        },
        GameInfo {
            name: "Geometry Dash".into(),
            id: GameId(1),
            gdrive_id: "1vUtZqg6eDpRlKWNzDkjwq9_REN2LfYHt".into(),
            exe: "GeometryDash.exe".into(),
            hooks: script!("geometry_dash"),
        },
        GameInfo {
            name: "Bloons TD 6".into(),
            id: GameId(2),
            gdrive_id: "16paRX1A9qxvFyWzgGGremMviKWvT0fuN".into(),
            exe: "BloonsTD6.exe".into(),
            hooks: script!("bloons_td_6"),
        },
        GameInfo {
            name: "OMORI".into(),
            id: GameId(3),
            gdrive_id: "1liGmClrNJu2yPmX3HdyMoO_QXI64gJKy".into(),
            exe: "OMORI.exe".into(),
            hooks: script!("omori"),
        },
        GameInfo {
            name: "Five Nights at Freddy's".into(),
            id: GameId(4),
            gdrive_id: "1gnn4X4OtIVuB-t6ZblBN_UUD4NawbzJf".into(),
            exe: "FiveNightsatFreddys.exe".into(),
            hooks: script!("fnaf"),
        },
        GameInfo {
            name: "Five Nights at Freddy's 2".into(),
            id: GameId(5),
            gdrive_id: "1gPjMCiX5IHnhoXi18jGAbHf1CiMlzSEf".into(),
            exe: "FiveNightsatFreddys2.exe".into(),
            hooks: script!("fnaf2"),
        },
        GameInfo {
            name: "Portal".into(),
            id: GameId(6),
            gdrive_id: "14eIkhwzFvZ8T2jEVs_1loEv9A2TcZSP2".into(),
            exe: "Portal.exe".into(),
            hooks: script!("portal"),
        },
        GameInfo {
            name: "SUPERHOT".into(),
            id: GameId(7),
            gdrive_id: "1wCazjUEACX_o8G0_09fJLGFms0HCR3DY".into(),
            exe: "SUPERHOT.exe".into(),
            hooks: script!("superhot"),
        },
        GameInfo {
            name: "Untitled Goose Game".into(),
            id: GameId(8),
            gdrive_id: "1MAUhwh-1wEWSTQg5mK8f0X74oVzcg4NM".into(),
            exe: "Untitled.exe".into(),
            hooks: script!("goose"),
        },
        GameInfo {
            name: "UNDERTALE".into(),
            id: GameId(9),
            gdrive_id: "1mm-NCvlluUzf4MUVECl_XB1ZmK1Fvw7J".into(),
            exe: "UNDERTALE.exe".into(),
            hooks: script!("undertale"),
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

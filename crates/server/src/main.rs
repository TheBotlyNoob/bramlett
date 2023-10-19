use axum::{routing::get, Json, Router};
use common::GameInfo;
use std::path::PathBuf;

macro_rules! script {
    ($name: literal) => {
        include_str!(concat!("../scripts/", $name, ".rhai")).into()
    };
}

fn games() -> Vec<GameInfo> {
    vec![
        GameInfo {
            name: "Papers Please".into(),
            gdrive_id: "1HzLYxrQQ1a8wj89KwTXMQ_hwdE4lCLvr".into(),
            exe: PathBuf::from("PapersPlease.exe"),
            hooks: script!("papers_please"),
            args: vec![],
        },
        GameInfo {
            name: "Geometry Dash".into(),
            gdrive_id: "1vUtZqg6eDpRlKWNzDkjwq9_REN2LfYHt".into(),
            exe: PathBuf::from("GeometryDash.exe"),
            hooks: script!("geometry_dash"),
            args: vec![],
        },
        GameInfo {
            name: "Bloons TD 6".into(),
            gdrive_id: "16paRX1A9qxvFyWzgGGremMviKWvT0fuN".into(),
            exe: PathBuf::from("BloonsTD6.exe"),
            hooks: script!("bloons_td_6"),
            args: vec![],
        },
        GameInfo {
            name: "OMORI".into(),
            gdrive_id: "1liGmClrNJu2yPmX3HdyMoO_QXI64gJKy".into(),
            exe: PathBuf::from("OMORI.exe"),
            hooks: script!("omori"),
            args: vec![],
        },
        GameInfo {
            name: "Five Nights at Freddy's".into(),
            gdrive_id: "1gnn4X4OtIVuB-t6ZblBN_UUD4NawbzJf".into(),
            exe: PathBuf::from("FiveNightsatFreddys.exe"),
            hooks: script!("fnaf"),
            args: vec!["/NOF".into()],
        },
        GameInfo {
            name: "Five Nights at Freddy's 2".into(),
            gdrive_id: "1gPjMCiX5IHnhoXi18jGAbHf1CiMlzSEf".into(),
            exe: PathBuf::from("FiveNightsatFreddys2.exe"),
            hooks: script!("fnaf2"),
            args: vec![],
        },
        GameInfo {
            name: "Portal".into(),
            gdrive_id: "14eIkhwzFvZ8T2jEVs_1loEv9A2TcZSP2".into(),
            exe: PathBuf::from("Portal.exe"),
            hooks: script!("portal"),
            args: vec![],
        },
        GameInfo {
            name: "SUPERHOT".into(),
            gdrive_id: "1wCazjUEACX_o8G0_09fJLGFms0HCR3DY".into(),
            exe: PathBuf::from("SUPERHOT.exe"),
            hooks: script!("superhot"),
            args: vec![],
        },
        GameInfo {
            name: "Untitled Goose Game".into(),
            gdrive_id: "1MAUhwh-1wEWSTQg5mK8f0X74oVzcg4NM".into(),
            exe: PathBuf::from("Untitled.exe"),
            hooks: script!("goose"),
            args: vec![],
        },
        GameInfo {
            name: "Undertale".into(),
            gdrive_id: "1mm-NCvlluUzf4MUVECl_XB1ZmK1Fvw7J".into(),
            exe: PathBuf::from("UNDERTALE.exe"),
            hooks: script!("undertale"),
            args: vec![],
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

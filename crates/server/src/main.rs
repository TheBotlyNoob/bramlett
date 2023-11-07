use axum::{routing::get, Json, Router};
use common::{GameId, GameInfo};

macro_rules! script {
    ($name: literal) => {
        // TODO: actually do this
        String::new()
    };
}

fn games() -> Vec<GameInfo> {
    vec![
        GameInfo {
            name: "Papers Please".into(),
            id: GameId(0),
            gdrive_id: "1sBkd4vADCCH3WmoF6DqtKhjEt7G3D307".into(),
            exe: "PapersPlease.exe".into(),
            hooks: script!("papers_please"),
        },
        GameInfo {
            name: "Geometry Dash".into(),
            id: GameId(1),
            gdrive_id: "16CYi7pAMTsmbVmlEtUBOkLjvpqtQdght".into(),
            exe: "GeometryDash.exe".into(),
            hooks: script!("geometry_dash"),
        },
        GameInfo {
            name: "Bloons TD 6".into(),
            id: GameId(2),
            gdrive_id: "1yd62Hz-e4d_Z0gilXc18dRRmoD5o0WSY".into(),
            exe: "BloonsTD6.exe".into(),
            hooks: script!("bloons_td_6"),
        },
        GameInfo {
            name: "OMORI".into(),
            id: GameId(3),
            gdrive_id: "13QjrN_I8ccliWslVMINW2HNSNVxsqkSp".into(),
            exe: "OMORI.exe".into(),
            hooks: script!("omori"),
        },
        GameInfo {
            name: "Totally Accurate Battle Simulator".into(),
            id: GameId(4),
            gdrive_id: "1KiuU7rf9BK6v3_TJTbRw_HLi9hUSWByJ".into(),
            exe: "TotallyAccurateBattleSimulator.exe".into(),
            hooks: script!("tabs"),
        },
        GameInfo {
            name: "Half-Life".into(),
            id: GameId(5),
            gdrive_id: "1TTHHjQWSu_KBwTv6ox7pppsFGd-8t95V".into(),
            exe: "hl.exe".into(),
            hooks: script!("half_life"),
        },
        GameInfo {
            name: "Call of Duty 2".into(),
            id: GameId(6),
            gdrive_id: "1GtNsZggdQkyLK8Seiem7KGhRIyFHFP7C".into(),
            exe: "CoD2SP_s.exe".into(),
            hooks: script!("cod2"),
        },
        GameInfo {
            name: "Portal".into(),
            id: GameId(7),
            gdrive_id: "1B_GDs711J30mCwMU8F12L8fBKX4dbYJM".into(),
            exe: "Portal.exe".into(),
            hooks: script!("portal"),
        },
        GameInfo {
            name: "SUPERHOT".into(),
            id: GameId(8),
            gdrive_id: "1qZSFbucjmEmhkbr9y93csZklYtkfzibK".into(),
            exe: "SUPERHOT.exe".into(),
            hooks: script!("superhot"),
        },
        GameInfo {
            name: "Five Nights at Freddy's".into(),
            id: GameId(9),
            gdrive_id: "1TddScp06i7Echbh-JAa_PKTce3GyBRhg".into(),
            exe: "FiveNightsatFreddys.exe".into(),
            hooks: script!("fnaf"),
        },
        GameInfo {
            name: "Five Nights at Freddy's 2".into(),
            id: GameId(10),
            gdrive_id: "1C-3vrgV0gaLeZ5b19WKRroQUJRkhs5AD".into(),
            exe: "FiveNightsatFreddys2.exe".into(),
            hooks: script!("fnaf2"),
        },
        // GameInfo {
        //     name: "Five Nights at Freddy's 3".into(),
        //     id: GameId(11),
        //     gdrive_id: todo!(),
        //     exe: "FiveNightsatFreddys3.exe".into(),
        //     hooks: script!("fnaf3"),
        // },
        // GameInfo {
        //     name: "Five Nights at Freddy's 4".into(),
        //     id: GameId(12),
        //     gdrive_id: todo!(),
        //     exe: "FiveNightsatFreddys4.exe".into(),
        //     hooks: script!("fnaf4"),
        // },
    ]
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { Json(games()) }))
        .into())
}

#[cfg(test)]
#[test]
fn assert_unique_ids() {
    let mut seen_ids = std::collections::HashSet::new();
    for game in games() {
        assert!(seen_ids.insert(game.id));
    }
}

use common::{GameId, GameInfo};
use warp::Filter;
use warp::Reply;

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
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/434529ee3a4ce2b39f5bce0502c444c3.png".into(),
            id: GameId(0),
            gdrive_id: "1sBkd4vADCCH3WmoF6DqtKhjEt7G3D307".into(),
            exe: "PapersPlease.exe".into(),
            hooks: script!("papers_please"),
        },
        GameInfo {
            name: "Geometry Dash".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/818d9d1eaede1c656acf1db6a10d9866.png".into(),
            id: GameId(1),
            gdrive_id: "16CYi7pAMTsmbVmlEtUBOkLjvpqtQdght".into(),
            exe: "steamapps/common/Geometry Dash/GeometryDash.exe".into(),
            hooks: script!("geometry_dash"),
        },
        GameInfo {
            name: "Bloons TD 6".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/b9ba7f4a80d4097aa09d1a2a13fdd183.png".into(),
            id: GameId(2),
            gdrive_id: "1yd62Hz-e4d_Z0gilXc18dRRmoD5o0WSY".into(),
            exe: "BloonsTD6.exe".into(),
            hooks: script!("bloons_td_6"),
        },
        GameInfo {
            name: "OMORI".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/1f892957b260a88f9b21ab438a520ff1.webp".into(), // I would LOVE a custom icon if you've finished the game
            id: GameId(3),
            gdrive_id: "13QjrN_I8ccliWslVMINW2HNSNVxsqkSp".into(),
            exe: "OMORI.exe".into(),
            hooks: script!("omori"),
        },
        GameInfo {
            name: "Totally Accurate Battle Simulator".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/fdd570402b33973a780e64fdce5c391e.png".into(),
            id: GameId(4),
            gdrive_id: "1KiuU7rf9BK6v3_TJTbRw_HLi9hUSWByJ".into(),
            exe: "TotallyAccurateBattleSimulator.exe".into(),
            hooks: script!("tabs"),
        },
        GameInfo {
            name: "Half-Life".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/10f16ffcd88e1cf00e43178442437763.png".into(),
            id: GameId(5),
            gdrive_id: "1TTHHjQWSu_KBwTv6ox7pppsFGd-8t95V".into(),
            exe: "hl.exe".into(),
            hooks: script!("half_life"),
        },
        GameInfo {
            name: "Call of Duty 2".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/8a563489dff3761a5f856d56d8937c95.jpg".into(),
            id: GameId(6),
            gdrive_id: "1GtNsZggdQkyLK8Seiem7KGhRIyFHFP7C".into(),
            exe: "CoD2SP_s.exe".into(),
            hooks: script!("cod2"),
        },
        GameInfo {
            name: "Portal".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/f772a46e9df669d0a39f78213b6685dc.webp".into(),
            id: GameId(7),
            gdrive_id: "1B_GDs711J30mCwMU8F12L8fBKX4dbYJM".into(),
            exe: "Portal.exe".into(),
            hooks: script!("portal"),
        },
        GameInfo {
            name: "SUPERHOT".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/33ce538f76f7a4060a93caa7bfd6c8c3.png".into(),
            id: GameId(8),
            gdrive_id: "1qZSFbucjmEmhkbr9y93csZklYtkfzibK".into(),
            exe: "SUPERHOT.exe".into(),
            hooks: script!("superhot"),
        },
        GameInfo {
            name: "Five Nights at Freddy's".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/0ede7c7ae62e005507fc15cd016c3fdf.png".into(),
            id: GameId(9),
            gdrive_id: "1TddScp06i7Echbh-JAa_PKTce3GyBRhg".into(),
            exe: "FiveNightsatFreddys.exe".into(),
            hooks: script!("fnaf"),
        },
        GameInfo {
            name: "Five Nights at Freddy's 2".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/ea2621147c2f3912a780587ee03c3889.png".into(),
            id: GameId(10),
            gdrive_id: "1C-3vrgV0gaLeZ5b19WKRroQUJRkhs5AD".into(),
            exe: "FiveNightsatFreddys2.exe".into(),
            hooks: script!("fnaf2"),
        },
        GameInfo {
            name: "Five Nights at Freddy's 3".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/ca6bb18482982a6fd1c6c6e74d7dae9e.png".into(),
            id: GameId(11),
            gdrive_id: "1Qu2_VRYU_Fm1_gDLw264oVzvfIjw8f42".into(),
            exe: "FiveNightsatFreddys3.exe".into(),
            hooks: script!("fnaf3"),
        },
        GameInfo {
            name: "Five Nights at Freddy's 4".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/cba7dbaf4c4cb512fdb03f79f3e02693.png".into(),
            id: GameId(12),
            gdrive_id: "1Q2KZvvSimGWjDWmRnvvqcGZMtqjw2EM3".into(),
            exe: "FiveNightsatFreddys4.exe".into(),
            hooks: script!("fnaf4"),
        },
        GameInfo {
            name: "UNDERTALE".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/49da5f75021bacfc161ee7fb005d65a6.png".into(),
            id: GameId(13),
            gdrive_id: "1DEnp81K_zmy-l5CRsyXCl6gVKCNakwiN".into(),
            exe: "Undertale.exe".into(),
            hooks: script!("undertale"),
        },
        GameInfo {
            name: "Call of Duty".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/d50024a382b0bbae0ba02d1ef479b927.jpg".into(),
            id: GameId(14),
            gdrive_id: "1Dxxdcj7sDNgNcPfcqI_-5ZBYK-lNCywI".into(), 
            exe: "CoDSP.exe".into(),
            hooks: script!("cod"),
        },
        // GameInfo {
        //     name: "Call of Duty 4: Modern Warfare".into(),
        //     icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/b3049ee4f30b41fa9f41d88a0068f65c.png".into(),
        //     id: GameId(15),
        //     gdrive_id: "1GtNsZggdQkyLK8Seiem7KGhRIyFHFP7C".into(), // fix this
        //     exe: "iw3sp.exe".into(), // fix this
        //     hooks: script!("cod4"),
        // },
        // GameInfo {
        //     name: "Call of Duty: World at War".into(),
        //     icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/c9aa2f73002e81f5e059ed1184113a44.png".into(),
        //     id: GameId(16),
        //     gdrive_id: "1GtNsZggdQkyLK8Seiem7KGhRIyFHFP7C".into(), // fix this
        //     exe: "CoDWaW.exe".into(), // fix this
        //     hooks: script!("cod_waw"),
        // } 
        GameInfo {
            name: "Grand Theft Auto: San Andreas".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/4b7890d268495230ee3f9bcd05ce3365.png".into(),
            id: GameId(17),
            gdrive_id: "14k2cdhdigdgB3mDSSveOJmTBLVCtwfpD".into(),
            exe: "gta_sa.exe".into(),
            hooks: script!("gta_sa"),
        },
        GameInfo {
            name: "Grand Theft Auto: Vice City".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/4eb6720c6cd70ee9e67ca6e4dc12e3df.png".into(),
            id: GameId(18),
            gdrive_id: "1M4DlCIqeOvevPZ7aLUNLDiiGAqyxa5ct".into(),
            exe: "gta-vc.exe".into(),
            hooks: script!("gta_vc"),
        },
        GameInfo {
            name: "Getting Over It".into(),
            icon: "https://cdn2.steamgriddb.com/file/sgdb-cdn/grid/8d3c5c9e20f9162501d14594445a7150.png".into(),
            id: GameId(19),
            gdrive_id: "13feVmY99R_xQIQtFPRnyUIFgIjcHoWKP".into(),
            exe: "GettingOverIt.exe".into(), // fix this
            hooks: script!("getting_over_it"),
        }
    ]
}

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let route = warp::get()
        .and(
            warp::path("games")
                .and(warp::path::end())
                .map(|| warp::reply::json(&games())),
        )
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(["Content-Type", "User-Agent"])
                .allow_methods(["OPTIONS", "GET", "POST", "DELETE"]),
        );
    Ok(route.boxed().into())
}
#[cfg(test)]
#[test]
fn assert_unique_ids() {
    let mut seen_ids = std::collections::HashSet::new();
    for game in games() {
        assert!(seen_ids.insert(game.id));
    }
}

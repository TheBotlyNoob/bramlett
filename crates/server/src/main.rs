use axum::{routing::get, Json, Router};
use common::GameInfo;
use std::net::SocketAddr;

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
            gdrive_id: todo!(),
        },
        GameInfo {
            name: "OMORI".into(),
            gdrive_id: todo!(),
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

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { Json(games()) }));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on http://localhost:80");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

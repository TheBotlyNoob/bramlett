use std::{path::PathBuf, sync::Arc, time::Duration};

use async_stream::stream;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
};
use rspc::{integrations::httpz::Request, Config, Router};
use tokio::time::sleep;
use tower_http::cors::{AllowOrigin, CorsLayer};

struct Ctx {}

fn router() -> Arc<Router<Ctx>> {
    static ROUTER: once_cell::sync::Lazy<Arc<Router<Ctx>>> = once_cell::sync::Lazy::new(|| {
        rspc::Router::<Ctx>::new()
            .config(Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts"),
            ))
            .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
            .query("echo", |t| t(|_, v: String| v))
            .query("error", |t| {
                t(|_, _: ()| {
                    Err(rspc::Error::new(
                        rspc::ErrorCode::InternalServerError,
                        "Something went wrong".into(),
                    )) as Result<String, rspc::Error>
                })
            })
            .query("transformMe", |t| t(|_, _: ()| "Hello, world!".to_string()))
            .mutation("sendMsg", |t| {
                t(|_, v: String| {
                    println!("Client said '{}'", v);
                    v
                })
            })
            .subscription("pings", |t| {
                t(|_ctx, _args: ()| {
                    stream! {
                        println!("Client subscribed to 'pings'");
                        for i in 0..5 {
                            println!("Sending ping {}", i);
                            yield "ping".to_string();
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                })
            })
            // TODO: Results being returned from subscriptions
            // .subscription("errorPings", |t| t(|_ctx, _args: ()| {
            //     stream! {
            //         for i in 0..5 {
            //             yield Ok("ping".to_string());
            //             sleep(Duration::from_secs(1)).await;
            //         }
            //         yield Err(rspc::Error::new(ErrorCode::InternalServerError, "Something went wrong".into()));
            //     }
            // }))
            .build()
            .arced()
    });
    ROUTER.clone()
}

#[tokio::main]
async fn main() {
    // We disable CORS because this is just an example. DON'T DO THIS IN PRODUCTION!
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
        ]));

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest(
            "/rspc",
            router()
                .endpoint(|req: Request| {
                    println!("Client requested operation '{}'", req.uri().path());
                    Ctx {}
                })
                .axum(),
        )
        .layer(cors);

    let addr = "[::]:4000".parse::<std::net::SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("listening on http://{}/rspc/version", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

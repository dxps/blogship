// use blogship_srv::server::new_server;

// #[rocket::main]
// async fn main() {
//     // Server is instantiated and started in one step.
//     let server = new_server().await;
//     if server.is_err() {
//         println!("Server error: {}", server.err().unwrap())
//     }
// }

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use tower::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    tracing_subscriber::fmt::init();
    let tracing_layer = TraceLayer::new_for_http();

    let http_svc = Router::new()
        .route("/api/hello", get(hello))
        .fallback(get(|req| async move {
            match ServeDir::new(opt.static_dir).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(tracing_layer)
        .into_make_service();

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    log::info!("Listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(http_svc)
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello"
}

#[derive(Parser, Debug)]
#[clap(name = "blogshipd", about = "The server part of BlogShip system")]
struct Opt {
    /// the HTTP listening address
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// the HTTP listening port
    #[clap(short = 'p', long = "port", default_value = "8090")]
    port: u16,

    /// the logging level
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,

    /// the directory where static files are served from
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

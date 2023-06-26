use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}

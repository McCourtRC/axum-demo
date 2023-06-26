use std::env;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let port = env::var("PORT").expect("missing PORT env var");
    let addr = format!("0.0.0.0:{}", port);

    println!("listening on addr {:?}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

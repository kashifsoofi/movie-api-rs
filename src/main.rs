use axum::{routing::get, Router};
use std::net::SocketAddr;

mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(controllers::health::get));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

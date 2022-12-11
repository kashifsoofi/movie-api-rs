use axum::{routing::get, Router};
use std::net::{SocketAddr, SocketAddrV4, IpAddr};

use movie_api::configuration::get_configuration;
use movie_api::telemetry::{get_subscriber, init_subscriber};

mod controllers;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("movie-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let app = Router::new().route("/health", get(controllers::health::get));

    let ip_addr: IpAddr = configuration.http_server.host.parse().expect("invalid ip address");
    let addr = SocketAddr::from((ip_addr, configuration.http_server.port));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

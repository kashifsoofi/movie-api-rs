use crate::configuration::{Configuration, DatabaseConfiguration};
use crate::controllers::health;
use axum::{routing::get, Router};
use tracing::log::LevelFilter;
use std::str::FromStr;
use std::time::Duration;
use std::net::SocketAddr;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{PgPool, ConnectOptions};
use tokio::signal;

pub struct Application {
    socket_addr: SocketAddr,
    app: Router,
}

impl Application {
    pub async fn build(configuration: Configuration) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.http_server.host, configuration.http_server.port
        );
        let socket_addr: SocketAddr = address.parse().expect("invalid host address");

        let app = app(connection_pool);

        Ok(Self{ socket_addr, app })
    }

    pub async fn run_until_stopped(self) {
        tracing::debug!("listening on {}", self.socket_addr);
        axum::Server::bind(&self.socket_addr)
            .serve(self.app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    }
}

pub fn app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health::get))
        .with_state(db_pool)
}

pub fn get_connection_pool(configuration: &DatabaseConfiguration) -> PgPool {
    let mut connect_options = PgConnectOptions::from_str(&configuration.database_url).expect("invalid connection string");
    let log_level = LevelFilter::from_str(&configuration.log_level).unwrap_or_else(|_| LevelFilter::Error);
    connect_options.log_statements(log_level);

    PgPoolOptions::new()
        .max_connections(configuration.max_open_connections)
        .acquire_timeout(Duration::from_secs(3))
        .connect_lazy_with(connect_options)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

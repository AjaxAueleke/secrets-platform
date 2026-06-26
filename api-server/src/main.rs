use crate::db::connect;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod config;
mod controller;
mod db;
mod repository;
mod routes;
mod service;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = Config::from_env()?;
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .with(
            fmt::layer()
                .json()
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let pool = connect(&config.db_url).await?;
    let state = state::AppState { pgpool: pool };

    let app = Router::new()
        .route("/health-check", get(controller::healthcheck::healthcheck))
        .nest("/secrets", routes::secret::secret_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .unwrap();
    info!("Server started on {:?}", listener.local_addr());
    axum::serve(listener, app).await?;
    Ok(())
}

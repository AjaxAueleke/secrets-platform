use crate::db::connect;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use dotenvy::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod controller;
mod db;
mod repository;
mod routes;
mod service;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
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
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect(&db_url).await?;
    let state = state::AppState { pgpool: pool };

    // build our application with a route
    let app = Router::new()
        .route("/health-check", get(controller::healthcheck::healthcheck))
        .nest("/secrets", routes::secret::secret_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server started on {:?}", listener.local_addr());
    axum::serve(listener, app).await?;
    Ok(())
}

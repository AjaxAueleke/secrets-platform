use dotenvy::dotenv;
use std::env;
use crate::db::connect;
use axum::{
    routing::get,
    Router
};
use sqlx::Executor;
mod db;
mod state;
mod routes;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connect(&db_url).await?;
    let state = state::AppState {
        pgpool: pool,
    };

    // build our application with a route
    let app = Router::new()
        .route("/health-check", get("healthcheck")).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}


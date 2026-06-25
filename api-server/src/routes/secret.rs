use crate::controller::secret::create_secret;
use crate::state::AppState;
use axum::{Router, routing::post};

pub(crate) fn secret_router() -> Router<AppState> {
    Router::new().route("/", post(create_secret))
}

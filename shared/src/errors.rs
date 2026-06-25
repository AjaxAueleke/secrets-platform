use thiserror::Error;

#[cfg(feature = "axum")]
use axum::http::StatusCode;
#[cfg(feature = "axum")]
use axum::response::{IntoResponse, Response};

#[derive(Error, Debug)]
pub enum AppError {
    #[cfg(feature = "db")]
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error: {0}")]
    InternalError(#[from] anyhow::Error),
}

#[cfg(feature = "axum")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            #[cfg(feature = "db")]
            AppError::DatabaseError(_err) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            AppError::SerializationError(_err) => (StatusCode::BAD_REQUEST).into_response(),
            AppError::ValidationError(_err) => (StatusCode::BAD_REQUEST).into_response(),
            AppError::InternalError(_err) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

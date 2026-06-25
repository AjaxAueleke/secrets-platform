use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use shared::AppError;
use shared::CreateSecretRequest;
use shared::models::Secret;
use sqlx::types::chrono;

pub async fn create_secret(
    State(_state): State<AppState>,
    payload: Result<Json<CreateSecretRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<Secret>), AppError> {
    let Json(data) =
        payload.map_err(|_| AppError::SerializationError("Invalid json payload".to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(Secret {
            id: 1,
            key_name: data.key_name,
            service_name: data.service_name,
            key_value: data.key_value.into_bytes(),
            version: 1,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        }),
    ))
}

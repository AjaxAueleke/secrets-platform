#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Secret {
    pub id: i64,
    pub key_name: String,
    pub service_name: String,
    pub key_value: Vec<u8>,
    pub version: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

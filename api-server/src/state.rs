use sqlx::PgPool;
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub pgpool: PgPool,
}

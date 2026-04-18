use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState {
     pub pgpool: PgPool,
}

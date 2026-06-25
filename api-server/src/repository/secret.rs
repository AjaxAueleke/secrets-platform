use shared::AppError;
use shared::models::Secret;
use sqlx::PgPool;

#[allow(dead_code)]
pub struct SecretRepository<'a> {
    pool: &'a PgPool,
}

#[allow(dead_code)]
impl<'a> SecretRepository<'a> {
    pub async fn insert(self, _: &'a Secret) -> Result<Secret, AppError> {
        todo!("Need to implement secret data")
    }
}

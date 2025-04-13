use sqlx::{PgPool, PgTransaction};

pub struct Repo {
    pool: PgPool,
}

impl Repo {
    pub async fn new(url: &str) -> sqlx::Result<Self> {
        let pool = PgPool::connect(url).await?;

        Ok(Self { pool })
    }

    pub async fn transaction(&self) -> sqlx::Result<PgTransaction> {
        self.pool.begin().await
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

pub mod remarks;
pub mod tags;

use canopus_repository::Repository;
use eyre::WrapErr;
use sqlx::PgPool;
use std::env;

pub struct Engine {
    repository: Repository,
}

impl Engine {
    pub async fn start() -> eyre::Result<Self> {
        dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

        let database_url =
            env::var("DATABASE_URL").wrap_err_with(|| "Missing DATABASE_URL env var")?;

        let pool = PgPool::connect(&database_url)
            .await
            .wrap_err_with(|| "Failed to connect to database")?;

        Ok(Self {
            repository: Repository { pool },
        })
    }
}

pub mod remarks;
pub mod tags;

use canopus_definitions::Result;
use canopus_repository::Repository;
use sqlx::PgPool;
use std::env;

pub struct Engine {
    repository: Repository,
}

impl Engine {
    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    pub async fn start() -> Result<Self> {
        initialize_environment()?;

        Ok(Self {
            repository: initialize_repository().await?,
        })
    }
}

fn initialize_environment() -> Result<()> {
    dotenvy::dotenv().map_err(|err| eyre::Error::from(err).wrap_err("Failed to load .env file"))?;

    Ok(())
}

async fn initialize_repository() -> Result<Repository> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|err| eyre::Error::from(err).wrap_err("Missing DATABASE_URL"))?;

    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|err| eyre::Error::from(err).wrap_err("Failed to connect to database"))?;

    Ok(Repository { pool })
}

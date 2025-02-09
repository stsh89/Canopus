pub mod operations;

use canopus_repository::Repository;
use sqlx::PgPool;
use std::env;

pub struct Engine {
    repository: Repository,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    EnvironmentVariable(#[source] eyre::Error),

    #[error("{0}")]
    File(#[source] eyre::Error),

    #[error("{0}")]
    Repository(#[source] eyre::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

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
    dotenvy::dotenv().map_err(|err| {
        Error::File(eyre::Error::from(err).wrap_err("Failed to load .env file"))
    })?;

    Ok(())
}

async fn initialize_repository() -> Result<Repository> {
    let database_url = env::var("DATABASE_URL").map_err(|err| {
        Error::EnvironmentVariable(eyre::Error::from(err).wrap_err("Missing DATABASE_URL"))
    })?;

    let pool = PgPool::connect(&database_url).await.map_err(|err| {
        Error::Repository(eyre::Error::from(err).wrap_err("Failed to connect to database"))
    })?;

    Ok(Repository { pool })
}

impl From<canopus_protocol::Error> for Error {
    fn from(err: canopus_protocol::Error) -> Self {
        match err {
            canopus_protocol::Error::Repository(report) => Error::Repository(report),
        }
    }
}

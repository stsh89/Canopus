mod brands;
mod cli;
mod error;
mod repo;
mod web;

use clap::Parser;
use cli::Cli;
use repo::Repo;

pub use error::Error as AppError;
use web::ServerConfig;
type AppResult<T> = std::result::Result<T, AppError>;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let Cli { database_url, port } = Cli::parse();

    let repo = Repo::new(&database_url).await?;

    web::run(repo, ServerConfig { port }).await?;

    Ok(())
}

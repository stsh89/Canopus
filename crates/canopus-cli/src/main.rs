mod cli;
mod session;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().execute().await?;

    Ok(())
}

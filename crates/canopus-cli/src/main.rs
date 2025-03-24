use canopus_cli::{App, Cli};
use clap::Parser;
use eyre::WrapErr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    App::initialize()?.execute(cli).await?;

    Ok(())
}

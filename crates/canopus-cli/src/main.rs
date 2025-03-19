use canopus_cli::{App, Cli};
use eyre::Context;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

    let cli = Cli::new()?;

    App::initialize()?.execute(cli).await?;

    Ok(())
}

mod commands;

use canopus_cli::CliState;
use canopus_definitions::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{}", e);
    }
}

async fn try_main() -> Result<()> {
    dotenvy::dotenv().map_err(Into::<eyre::Error>::into)?;

    let state = CliState::new()?;
    canopus_cli::run(&state).await?;

    Ok(())
}

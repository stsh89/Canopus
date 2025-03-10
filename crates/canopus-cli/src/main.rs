// mod formatter;
// mod session;
// mod tags;
mod commands;
mod display;

use canopus_client::{Client, tags};
use canopus_definitions::Result;
use clap::Parser;
use commands::Commands;
use display::Renderer;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{}", e);
    }
}

async fn try_main() -> Result<()> {
    let Cli { command } = Cli::parse();

    let client = Client::new()?;

    let renderer = Renderer::new();

    match command {
        Commands::ShowTag { id } => {
            let tag = tags::show(&client, id).await?;

            renderer.render(tag);
        }
        Commands::ListTags { page_token } => {
            let page = tags::index(&client, page_token).await?;

            renderer.render(page);
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// mod formatter;
// mod session;
// mod tags;
mod commands;
mod display;
mod error;

use canopus_client::Client;
use clap::{Parser, ValueEnum};
use commands::Commands;
use display::{RenderOptions, Renderer};

type Result<T> = std::result::Result<T, error::Error>;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{}", e);
    }
}

async fn try_main() -> Result<()> {
    let Cli { command, format } = Cli::parse();

    let client = Client::new()?;

    let renderer = Renderer::new(RenderOptions {
        format: format.into(),
    });

    match command {
        Commands::ShowTag { id } => {
            let tag = client.show_tag(id).await?;

            renderer.render(tag);
        }
        Commands::ListTags { page_token } => {
            let page = client.list_tags(page_token).await?;

            renderer.render(page);
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Text,
    Json,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Text => f.write_str("text"),
            Format::Json => f.write_str("json"),
        }
    }
}

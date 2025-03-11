mod commands;
mod display;

use canopus_client::{Client, tags};
use canopus_definitions::{ApplicationError, Result};
use clap::{Parser, error::ErrorKind};
use commands::Commands;
use display::Renderer;
use std::env;

pub struct CliState {
    client: Client,
    renderer: Renderer,
}

impl CliState {
    pub fn new() -> Result<Self> {
        let base_url = env::var("API_URL").map_err(Into::<eyre::Error>::into)?;
        let client = Client::new(&base_url)?;
        let renderer = Renderer::new();

        Ok(Self { client, renderer })
    }
}

pub async fn run(state: &CliState) -> Result<()> {
    let cli = Cli::try_parse().map_err(map_clap_error)?;

    execute(cli, state).await?;

    Ok(())
}

pub async fn run_from(state: &CliState, args: Vec<&str>) -> Result<()> {
    let cli = Cli::try_parse_from(args).map_err(map_clap_error)?;

    execute(cli, state).await?;

    Ok(())
}

async fn execute(cli: Cli, state: &CliState) -> Result<()> {
    let Cli { command } = cli;
    let CliState { client, renderer } = state;

    match command {
        Commands::ShowTag { id } => {
            let tag = tags::show(client, id).await?;

            renderer.render(tag);
        }
        Commands::ListTags { page_token } => {
            let page = tags::index(client, page_token).await?;

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

fn map_clap_error(err: clap::Error) -> ApplicationError {
    match err.kind() {
        ErrorKind::InvalidSubcommand
        | ErrorKind::ValueValidation
        | ErrorKind::InvalidValue
        | ErrorKind::InvalidUtf8
        | ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion => ApplicationError::InvalidArgument(err.to_string()),
        _ => eyre::Error::from(err).into(),
    }
}

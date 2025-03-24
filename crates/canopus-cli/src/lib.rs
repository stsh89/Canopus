mod commands;
mod display;
mod editor;

use canopus_client::Client;
use canopus_definitions::{ApplicationError, ApplicationResult};
use clap::{Parser, Subcommand};
use commands::{RemarksCommands, TagsCommands};
use display::Renderer;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(flatten)]
    Tags(TagsCommands),

    #[command(flatten)]
    Remarks(RemarksCommands),
}

impl Cli {
    pub fn new_with_args(args: &[String]) -> ApplicationResult<Self> {
        let cli =
            Self::try_parse_from(args).map_err(|err| ApplicationError::msg(&err.to_string()))?;

        Ok(cli)
    }
}

pub struct App {
    client: Client,
    renderer: Renderer,
}

impl App {
    pub fn initialize() -> ApplicationResult<Self> {
        let client = Client::from_env()?;
        let renderer = Renderer::new();

        Ok(Self { client, renderer })
    }

    pub async fn execute(&self, cli: Cli) -> ApplicationResult<()> {
        let Cli { command } = cli;

        match command {
            Commands::Tags(command) => command.execute(self).await?,
            Commands::Remarks(command) => command.execute(self).await?,
        }

        Ok(())
    }
}

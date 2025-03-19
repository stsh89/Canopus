mod display;

use canopus_client::{Client, tags};
use canopus_definitions::ApplicationResult;
use clap::{Parser, Subcommand};
use display::Renderer;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ShowTag {
        id: Uuid,
    },
    ListTags {
        #[arg(short, long)]
        page_token: Option<String>,
    },
}

impl Cli {
    pub fn new_with_args(args: &[&str]) -> eyre::Result<Self> {
        let cli = Self::try_parse_from(args)?;

        Ok(cli)
    }
}

pub struct CliContext {
    client: Client,
    renderer: Renderer,
}

impl CliContext {
    pub fn initialize() -> eyre::Result<Self> {
        let client = Client::from_env()?;
        let renderer = Renderer::new();

        Ok(Self { client, renderer })
    }

    pub async fn execute(&self, cli: Cli) -> ApplicationResult<()> {
        let Cli { command } = cli;
        let CliContext { client, renderer } = self;

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
}

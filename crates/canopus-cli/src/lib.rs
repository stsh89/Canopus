mod cli;
mod display;

pub use cli::*;

use canopus_client::{Client, tags};
use canopus_definitions::ApplicationResult;
use display::Renderer;

pub struct App {
    client: Client,
    renderer: Renderer,
}

impl App {
    pub fn initialize() -> eyre::Result<Self> {
        let client = Client::from_env()?;
        let renderer = Renderer::new();

        Ok(Self { client, renderer })
    }

    pub async fn execute(&self, cli: Cli) -> ApplicationResult<()> {
        let Cli { command } = cli;
        let App { client, renderer } = self;

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

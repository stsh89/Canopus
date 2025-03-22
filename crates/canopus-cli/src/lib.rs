mod display;
mod editor;

use canopus_client::{
    Client,
    remarks::{self, NewRemark, RemarkUpdates},
    tags,
};
use canopus_definitions::ApplicationResult;
use clap::{Parser, Subcommand};
use display::Renderer;
use uuid::Uuid;

const SUBSYSTEM_NAME: &str = "Cli";

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    CreateRemark {
        #[arg(short, long)]
        tags: Vec<String>,
    },
    DeleteRemark {
        id: Uuid,
    },
    EditRemark {
        id: Uuid,
    },
    ShowRemark {
        id: Uuid,
    },
    ShowTag {
        id: Uuid,
    },
    ListTags {
        #[arg(short, long)]
        page_token: Option<String>,
    },
    ListRemarks {
        #[arg(short, long)]
        page_token: Option<String>,
    },
    UpdateRemark {
        id: Uuid,
        #[arg(short, long, value_delimiter = ',', num_args = 1..)]
        tags: Option<Vec<String>>,
        #[arg(short, long)]
        essence: Option<String>,
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
            Commands::CreateRemark { tags } => {
                let essence = editor::open()?;

                let remark = remarks::create(client, NewRemark { essence, tags }).await?;

                renderer.render(remark);
            }
            Commands::DeleteRemark { id } => {
                let remark = remarks::delete(client, id).await?;

                renderer.render(remark);
            }
            Commands::EditRemark { id } => {
                let remark = remarks::show(client, id).await?;
                let essence = editor::edit(remark.essence())?;

                let remark = remarks::update(
                    client,
                    id,
                    RemarkUpdates {
                        essence: Some(essence),
                        ..Default::default()
                    },
                )
                .await?;

                renderer.render(remark);
            }
            Commands::ShowRemark { id } => {
                let remark = remarks::show(client, id).await?;

                renderer.render(remark);
            }
            Commands::ShowTag { id } => {
                let tag = tags::show(client, id).await?;

                renderer.render(tag);
            }
            Commands::ListTags { page_token } => {
                let page = tags::index(client, page_token).await?;

                renderer.render(page);
            }
            Commands::ListRemarks { page_token } => {
                let page = remarks::index(client, page_token).await?;

                renderer.render(page);
            }
            Commands::UpdateRemark { id, essence, tags } => {
                let remark = remarks::update(client, id, RemarkUpdates { essence, tags }).await?;

                renderer.render(remark);
            }
        }

        Ok(())
    }
}

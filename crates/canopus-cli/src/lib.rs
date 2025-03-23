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
        essence: String,

        #[arg(short, long)]
        tags: Vec<String>,
    },
    DeleteRemark {
        id: Uuid,
    },
    EditRemark {
        id: Uuid,
    },
    NewRemark,
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
    ClearRemarkTags {
        id: Uuid,
    },
    UpdateRemark {
        id: Uuid,

        #[arg(short, long)]
        essence: Option<String>,

        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },
    UpdateTag {
        id: Uuid,

        #[arg(short, long)]
        title: String,
    },
}

impl Cli {
    pub fn new_with_args(args: &[String]) -> eyre::Result<Self> {
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
            Commands::CreateRemark { essence, tags } => {
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
            Commands::NewRemark => {
                let essence = editor::open()?;

                let remark = remarks::create(
                    client,
                    NewRemark {
                        essence,
                        tags: vec![],
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
            Commands::ClearRemarkTags { id } => {
                let remark = remarks::update(
                    client,
                    id,
                    RemarkUpdates {
                        essence: None,
                        tags: Some(vec![]),
                    },
                )
                .await?;

                renderer.render(remark);
            }
            Commands::UpdateTag { id, title } => {
                let tag = tags::update(client, id, title).await?;

                renderer.render(tag);
            }
        }

        Ok(())
    }
}

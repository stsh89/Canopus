use crate::CliApp;
use canopus_client::tags;
use canopus_definitions::ApplicationResult;
use clap::{Subcommand, arg};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum TagsCommands {
    ListTags {
        #[arg(short, long)]
        page_token: Option<String>,
    },

    ShowTag {
        id: Uuid,
    },

    UpdateTag {
        id: Uuid,

        #[arg(short, long)]
        title: String,
    },
}

impl TagsCommands {
    pub async fn execute(self, app: &CliApp) -> ApplicationResult<()> {
        let CliApp { client, renderer } = app;

        match self {
            Self::ListTags { page_token } => {
                let page = tags::index(client, page_token).await?;

                renderer.render(page);
            }
            Self::ShowTag { id } => {
                let tag = tags::show(client, id).await?;

                renderer.render(tag);
            }
            Self::UpdateTag { id, title } => {
                let tag = tags::update(client, id, title).await?;

                renderer.render(tag);
            }
        }

        Ok(())
    }
}

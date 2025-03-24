use crate::{App, editor};
use canopus_client::remarks::{self, NewRemark, RemarkUpdates};
use canopus_definitions::ApplicationResult;
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum RemarksCommands {
    ClearRemarkTags {
        id: Uuid,
    },

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

    ListRemarks {
        #[arg(short, long)]
        page_token: Option<String>,
    },

    ShowRemark {
        id: Uuid,
    },

    UpdateRemark {
        id: Uuid,

        #[arg(short, long)]
        essence: Option<String>,

        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },
}

impl RemarksCommands {
    pub async fn execute(self, app: &App) -> ApplicationResult<()> {
        let App { client, renderer } = app;

        match self {
            Self::ClearRemarkTags { id } => {
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
            Self::CreateRemark { essence, tags } => {
                let remark = remarks::create(client, NewRemark { essence, tags }).await?;

                renderer.render(remark);
            }
            Self::DeleteRemark { id } => {
                let remark = remarks::delete(client, id).await?;

                renderer.render(remark);
            }
            Self::EditRemark { id } => {
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
            Self::NewRemark => {
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
            Self::ShowRemark { id } => {
                let remark = remarks::show(client, id).await?;

                renderer.render(remark);
            }
            Self::ListRemarks { page_token } => {
                let page = remarks::index(client, page_token).await?;

                renderer.render(page);
            }
            Self::UpdateRemark { id, essence, tags } => {
                let remark = remarks::update(client, id, RemarkUpdates { essence, tags }).await?;

                renderer.render(remark);
            }
        }

        Ok(())
    }
}

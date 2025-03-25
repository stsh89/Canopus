use crate::{App, editor};
use canopus_client::remarks::{self, NewRemark, RemarkUpdates};
use canopus_definitions::ApplicationResult;
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum RemarksCommands {
    AddRemarkTags {
        id: Uuid,

        #[arg(required = true, value_delimiter = ',')]
        tags: Vec<String>,
    },

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

    DeleteRemarkTags {
        id: Uuid,

        #[arg(required = true, value_delimiter = ',')]
        tags: Vec<String>,
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

    ShowLastRemark,

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
            Self::AddRemarkTags { id, tags } => {
                let remark = remarks::show(client, id).await?;

                let current_tags = remark.tags().iter().map(ToString::to_string).collect();

                let remark = remarks::update(
                    client,
                    id,
                    RemarkUpdates {
                        essence: None,
                        tags: Some([current_tags, tags].concat()),
                    },
                )
                .await?;

                renderer.render(remark);
            }
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
            Self::DeleteRemarkTags {
                id,
                tags: tags_to_remove,
            } => {
                let remark = remarks::show(client, id).await?;

                let mut tags: Vec<String> = remark.tags().iter().map(ToString::to_string).collect();

                tags.retain(|tag| !tags_to_remove.iter().any(|t| t == tag));

                let remark = remarks::update(
                    client,
                    id,
                    RemarkUpdates {
                        essence: None,
                        tags: Some(tags),
                    },
                )
                .await?;

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
            Self::ShowLastRemark => {
                let page = remarks::index(client, None).await?;

                if let Some(remark) = page.items.first() {
                    renderer.render(remark);
                }
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

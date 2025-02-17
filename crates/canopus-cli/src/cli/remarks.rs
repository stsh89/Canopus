use crate::{formatter, session::Session};
use canopus_engine::{
    remarks::{self, NewRemark, RemarkUpdates, RemarksListingParameters},
    Engine,
};
use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
pub struct DeleteRemarkArguments {
    #[arg(id = "ID", long, alias = "id")]
    id: Uuid,
}

#[derive(Parser)]
pub struct GetRemarkArguments {
    #[arg(id = "ID", long, alias = "id")]
    id: Uuid,
}

#[derive(Parser)]
pub struct ListRemarksArguments {
    #[arg(id = "LoadNextPage", long, alias = "loadnextpage")]
    load_next_page: bool,
}

#[derive(Parser)]
pub struct UpdateRemarkArguments {
    #[arg(id = "ID", long, alias = "id")]
    id: Uuid,

    #[arg(id = "Essence", long, alias = "essence")]
    essence: Option<String>,

    #[arg(id = "AddTag", long, alias = "addtag")]
    add_tags: Vec<String>,

    #[arg(id = "RemoveTag", long, alias = "removetag")]
    remove_tags: Vec<String>,
}

#[derive(Parser)]
pub struct NewRemarkArguments {
    #[arg(id = "Essence", long, alias = "essence")]
    essence: String,

    #[arg(id = "Tag", long, alias = "tag")]
    tags: Vec<String>,
}

pub async fn delete_remark(
    engine: &Engine,
    arguments: DeleteRemarkArguments,
) -> anyhow::Result<()> {
    let DeleteRemarkArguments { id } = arguments;

    remarks::delete_remark(engine, id).await?;

    Ok(())
}

pub async fn get_remark(engine: &Engine, arguments: GetRemarkArguments) -> anyhow::Result<()> {
    let GetRemarkArguments { id } = arguments;

    let remark = remarks::get_remark(engine, id).await?;

    formatter::write_remark(remark.into(), std::io::stdout())?;

    Ok(())
}

pub async fn new_remark(engine: &Engine, arguments: NewRemarkArguments) -> anyhow::Result<()> {
    let NewRemarkArguments { essence, tags } = arguments;

    let id = remarks::create_remark(engine, NewRemark { essence, tags }).await?;

    let remark = remarks::get_remark(engine, id).await?;

    formatter::write_remark(remark.into(), std::io::stdout())?;

    Ok(())
}

pub async fn list_remarks(
    engine: &Engine,
    session: &mut Session,
    arguments: ListRemarksArguments,
) -> anyhow::Result<()> {
    let ListRemarksArguments { load_next_page } = arguments;

    let mut parameters = RemarksListingParameters::default();

    if load_next_page {
        let Some(token) = session.remarks_listing_pagination_token() else {
            return Ok(());
        };

        parameters.pagination_token = Some(token.to_string());
    }

    let listing = remarks::list_remarks(engine, parameters).await?;

    let remarks = listing.remarks.into_iter().map(Into::into).collect();

    formatter::write_remarks_table(remarks, std::io::stdout())?;

    session.set_remarks_listing_pagination_token(listing.pagination_token);

    Ok(())
}

pub async fn update_remark(
    engine: &Engine,
    arguments: UpdateRemarkArguments,
) -> anyhow::Result<()> {
    let UpdateRemarkArguments {
        id,
        essence,
        add_tags,
        remove_tags,
    } = arguments;

    remarks::update_remark(
        engine,
        RemarkUpdates {
            id,
            essence,
            add_tags,
            remove_tags,
        },
    )
    .await?;

    let remark = remarks::get_remark(engine, id).await?;

    formatter::write_remark(remark.into(), std::io::stdout())?;

    Ok(())
}

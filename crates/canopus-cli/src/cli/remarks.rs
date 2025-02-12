use crate::cli::formatter;
use canopus_engine::{
    remarks::{self, NewRemark},
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

    get_remark(engine, GetRemarkArguments { id }).await?;

    Ok(())
}

use crate::formatter;
use canopus_engine::{tags, Engine};
use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
pub struct GetTagArguments {
    #[arg(id = "ID", long, alias = "id")]
    id: Uuid,
}

pub async fn get_tag(engine: &Engine, arguments: GetTagArguments) -> anyhow::Result<()> {
    let GetTagArguments { id } = arguments;

    let tag = tags::get_tag(engine, id).await?;

    formatter::write_tag(tag.into(), std::io::stdout())?;

    Ok(())
}

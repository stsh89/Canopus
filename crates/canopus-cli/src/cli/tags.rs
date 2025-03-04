use crate::{formatter, session::Session};
use canopus_engine::{
    tags::{self, TagsListingParameters}, Engine
};
use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
pub struct GetTagArguments {
    #[arg(id = "ID", long, alias = "id")]
    id: Uuid,
}

#[derive(Parser)]
pub struct ListTagsArguments {
    #[arg(id = "LoadNextPage", long, alias = "loadnextpage")]
    load_next_page: bool,
}

pub async fn get_tag(engine: &Engine, arguments: GetTagArguments) -> anyhow::Result<()> {
    let GetTagArguments { id } = arguments;

    let tag = tags::get_tag(engine, id).await?;

    formatter::write_tag(tag.into(), std::io::stdout())?;

    Ok(())
}

pub async fn list_tags(
    engine: &Engine,
    session: &mut Session,
    arguments: ListTagsArguments,
) -> anyhow::Result<()> {
    let ListTagsArguments { load_next_page } = arguments;

    let mut parameters = TagsListingParameters::default();

    if load_next_page {
        let Some(token) = session.tags_pagination_token() else {
            return Ok(());
        };

        parameters.pagination_token = Some(token.to_string());
    }

    let listing = tags::list_tags(engine, parameters).await?;

    let tags = listing.tags.into_iter().map(Into::into).collect();

    formatter::write_tags_table(tags, std::io::stdout())?;

    if let Some(token) = listing.pagination_token {
        session.set_tags_pagination_token(token);
    } else {
        session.clear_tags_pagination_token();
    }

    Ok(())
}

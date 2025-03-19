use canopus_definitions::{ApplicationResult, Page, Tag};
use canopus_operations::tags::{self, ListTagsParameters};
use uuid::Uuid;
use crate::Engine;

pub async fn get_tag(engine: &Engine, id: Uuid) -> ApplicationResult<Tag> {
    let Engine {
        repository,
    } = engine;

    let tag = tags::get_tag(id, repository).await?;

    Ok(tag)
}

pub async fn list_tags(
    engine: &Engine,
    parameters: ListTagsParameters,
) -> ApplicationResult<Page<Tag>> {
    let Engine {
        repository,
    } = engine;

    let page = tags::list_tags(parameters, repository).await?;

    Ok(page)
}

use crate::{Engine, Result};
use canopus_definitions::{Page, Tag};
pub use canopus_operations::tags::TagsListingParameters;
use canopus_operations::tags;
use uuid::Uuid;

pub async fn get_tag(engine: &Engine, id: Uuid) -> Result<Tag> {
    let repository = engine.repository();

    let tag = tags::get_tag(id, repository).await?;

    Ok(tag)
}

pub async fn list_tags(engine: &Engine, parameters: TagsListingParameters) -> Result<Page<Tag>> {
    let repository = engine.repository();

    let page = tags::list_tags(parameters, repository).await?;

    Ok(page)
}

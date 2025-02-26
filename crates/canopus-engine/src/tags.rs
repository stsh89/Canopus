pub use canopus_protocol::tags::{TagsListing, TagsListingParameters};

use crate::{Engine, Result};
use canopus_protocol::tags;
pub use canopus_protocol::tags::Tag;
use uuid::Uuid;

pub async fn get_tag(engine: &Engine, id: Uuid) -> Result<Tag> {
    let repository = engine.repository();

    let tag = tags::get_tag(id, repository).await?;

    Ok(tag)
}

pub async fn list_tags(engine: &Engine, parameters: TagsListingParameters) -> Result<TagsListing> {
    let repository = engine.repository();

    let remarks = tags::list_tags(parameters, repository).await?;

    Ok(remarks)
}

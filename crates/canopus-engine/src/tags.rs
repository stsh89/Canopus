use crate::{Engine, Result};
use canopus_protocol::tags;
pub use canopus_protocol::tags::Tag;
use uuid::Uuid;

pub async fn get_tag(engine: &Engine, id: Uuid) -> Result<Tag> {
    let repository = engine.repository();

    let tag = tags::get_tag(id, repository).await?;

    Ok(tag)
}

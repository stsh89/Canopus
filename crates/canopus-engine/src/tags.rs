use crate::Engine;
use canopus_definitions::{ApplicationResult, Page, Tag};
use canopus_operations::tags::{self, TagsPageParameters};
use uuid::Uuid;

pub async fn get_tag(engine: &Engine, id: Uuid) -> ApplicationResult<Tag> {
    let Engine { repository } = engine;

    tags::get_tag(id, repository).await
}

pub async fn list_tags(
    engine: &Engine,
    parameters: TagsPageParameters,
) -> ApplicationResult<Page<Tag>> {
    let Engine { repository } = engine;

    tags::list_tags(parameters, repository).await
}

pub async fn update_tag(engine: &Engine, id: Uuid, title: String) -> ApplicationResult<Tag> {
    let Engine { repository } = engine;

    tags::update_tag(id, title, repository).await
}

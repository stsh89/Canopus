use canopus_definitions::{ApplicationResult, Page, PageToken, Tag, TagTitle};
use std::future::Future;
use uuid::Uuid;

pub trait GetTag {
    fn get_tag(&self, tag_id: Uuid) -> impl Future<Output = ApplicationResult<Tag>>;
}

pub trait ListTags {
    fn list_tags(
        &self,
        parameters: TagsPageParameters,
    ) -> impl Future<Output = ApplicationResult<Page<Tag>>>;
}

pub trait UpdateTag {
    fn update_tag(&self, tag: &mut Tag) -> impl Future<Output = ApplicationResult<()>>;
}

#[derive(Default)]
pub struct TagsPageParameters {
    pub page_token: Option<PageToken>,
}

#[tracing::instrument(skip_all)]
pub async fn get_tag(id: Uuid, repository: &impl GetTag) -> ApplicationResult<Tag> {
    repository.get_tag(id).await
}

#[tracing::instrument(skip_all)]
pub async fn list_tags(
    parameters: TagsPageParameters,
    repository: &impl ListTags,
) -> ApplicationResult<Page<Tag>> {
    repository.list_tags(parameters).await
}

#[tracing::instrument(skip_all)]
pub async fn update_tag(
    id: Uuid,
    title: TagTitle,
    repository: &(impl UpdateTag + GetTag),
) -> ApplicationResult<Tag> {
    let mut tag = repository.get_tag(id).await?;

    tag.set_title(title);
    repository.update_tag(&mut tag).await?;

    Ok(tag)
}

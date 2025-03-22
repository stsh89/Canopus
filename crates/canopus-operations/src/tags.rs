use canopus_definitions::{ApplicationResult, Page, PageToken, Tag};
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

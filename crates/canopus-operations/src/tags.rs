use canopus_definitions::{Page, Result, Tag};
use std::future::Future;
use uuid::Uuid;

pub trait GetTag {
    fn get_tag(&self, tag_id: Uuid) -> impl Future<Output = Result<Tag>>;
}

pub trait ListTags {
    fn list_tags(&self, parameters: ListTagsParameters) -> impl Future<Output = Result<Page<Tag>>>;
}

#[derive(Default)]
pub struct ListTagsParameters {
    pub page_token: Option<String>,
}

pub async fn get_tag(id: Uuid, repository: &impl GetTag) -> Result<Tag> {
    repository.get_tag(id).await
}

pub async fn list_tags(
    parameters: ListTagsParameters,
    repository: &impl ListTags,
) -> Result<Page<Tag>> {
    repository.list_tags(parameters).await
}

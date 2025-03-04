use std::future::Future;
use canopus_definitions::{Tag, Result};
use uuid::Uuid;

pub trait GetTag {
    fn get_tag(&self, tag_id: Uuid) -> impl Future<Output = Result<Tag>>;
}

pub trait ListTags {
    fn list_tags(
        &self,
        parameters: TagsListingParameters,
    ) -> impl Future<Output = Result<TagsListing>>;
}

pub struct TagsListing {
    pub tags: Vec<Tag>,
    pub pagination_token: Option<String>,
}

#[derive(Default)]
pub struct TagsListingParameters {
    pub pagination_token: Option<String>,
}

pub async fn get_tag(id: Uuid, repository: &impl GetTag) -> Result<Tag> {
    repository.get_tag(id).await
}

pub async fn list_tags(
    parameters: TagsListingParameters,
    repository: &impl ListTags,
) -> Result<TagsListing> {
    repository.list_tags(parameters).await
}

use canopus_definitions::{ApplicationResult, Remark};
use std::future::Future;
use uuid::Uuid;

pub struct RemarksListing {
    pub remarks: Vec<Remark>,
    pub pagination_token: Option<String>,
}

pub struct RemarkUpdates {
    pub id: Uuid,
    pub essence: Option<String>,
    pub add_tags: Vec<String>,
    pub remove_tags: Vec<String>,
}

pub struct NewRemark {
    pub essence: String,
    pub tags: Vec<String>,
}

impl RemarkUpdates {
    fn is_empty(&self) -> bool {
        self.add_tags.is_empty() && self.remove_tags.is_empty() && self.essence.is_none()
    }
}

#[derive(Default)]
pub struct RemarksListingParameters {
    pub pagination_token: Option<String>,
}

pub trait DeleteRemark {
    fn delete_remark(&self, id: Uuid) -> impl Future<Output = ApplicationResult<()>>;
}

pub trait GetRemark {
    fn get_remark(&self, id: Uuid) -> impl Future<Output = ApplicationResult<Remark>>;
}

pub trait InsertRemark {
    fn insert_remark(&self, new_remark: NewRemark)
    -> impl Future<Output = ApplicationResult<Uuid>>;
}

pub trait UpdateRemark {
    fn update_remark(
        &self,
        parameters: RemarkUpdates,
    ) -> impl Future<Output = ApplicationResult<()>>;
}

pub trait ListRemarks {
    fn list_remarks(
        &self,
        listing_parameters: RemarksListingParameters,
    ) -> impl Future<Output = ApplicationResult<RemarksListing>>;
}

#[tracing::instrument(skip_all)]
pub async fn create_remark(
    new_remark: NewRemark,
    repository: &impl InsertRemark,
) -> ApplicationResult<Uuid> {
    let NewRemark { essence, tags } = new_remark;

    let new_remark = NewRemark {
        essence: sanitize_essence(essence),
        tags: tags.into_iter().map(sanitize_tag).collect(),
    };

    repository.insert_remark(new_remark).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_remark(id: Uuid, repository: &impl DeleteRemark) -> ApplicationResult<()> {
    repository.delete_remark(id).await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn get_remark(id: Uuid, repository: &impl GetRemark) -> ApplicationResult<Remark> {
    repository.get_remark(id).await
}

#[tracing::instrument(skip_all)]
pub async fn list_remarks(
    parameters: RemarksListingParameters,
    repository: &impl ListRemarks,
) -> ApplicationResult<RemarksListing> {
    repository.list_remarks(parameters).await
}

#[tracing::instrument(skip_all)]
pub async fn update_remark(
    parameters: RemarkUpdates,
    repository: &impl UpdateRemark,
) -> ApplicationResult<()> {
    if parameters.is_empty() {
        return Ok(());
    }

    repository.update_remark(parameters).await
}

fn sanitize_essence(essence: String) -> String {
    essence.trim().to_string()
}

fn sanitize_tag(tag: String) -> String {
    tag.trim().to_string()
}

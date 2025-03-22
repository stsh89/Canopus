use canopus_definitions::{ApplicationError, ApplicationResult, Page, PageToken, Remark, RemarkEssence, TagTitle};
use std::future::Future;
use uuid::Uuid;

pub struct NewRemark {
    pub essence: RemarkEssence,
    pub tags: Vec<TagTitle>,
}

pub struct NewRemarkAttributes {
    pub essence: String,
    pub tags: Vec<String>,
}

pub struct RemarkChanges {
    pub essence: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Default)]
pub struct RemarksPageParameters {
    pub page_token: Option<PageToken>,
}

pub trait DeleteRemark {
    fn delete_remark(&self, id: Uuid) -> impl Future<Output = ApplicationResult<()>>;
}

pub trait GetRemark {
    fn get_remark(&self, id: Uuid) -> impl Future<Output = ApplicationResult<Remark>>;
}

pub trait InsertRemark {
    fn insert_remark(&self, remark: NewRemark)
    -> impl Future<Output = ApplicationResult<Remark>>;
}

pub trait UpdateRemark {
    fn update_remark(
        &self,
        remark: &Remark,
    ) -> impl Future<Output = ApplicationResult<()>>;
}

pub trait ListRemarks {
    fn list_remarks(
        &self,
        parameters: RemarksPageParameters,
    ) -> impl Future<Output = ApplicationResult<Page<Remark>>>;
}

#[tracing::instrument(skip_all)]
pub async fn create_remark(
    attributes: NewRemarkAttributes,
    repository: &impl InsertRemark,
) -> ApplicationResult<Remark> {
    let new_remark = NewRemark::new(attributes)?;

    repository.insert_remark(new_remark).await
}

#[tracing::instrument(skip_all)]
pub async fn delete_remark(id: Uuid, repository: &(impl DeleteRemark + GetRemark)) -> ApplicationResult<Remark> {
    let remark = repository.get_remark(id).await?;

    repository.delete_remark(id).await?;

    Ok(remark)
}

#[tracing::instrument(skip_all)]
pub async fn get_remark(id: Uuid, repository: &impl GetRemark) -> ApplicationResult<Remark> {
    repository.get_remark(id).await
}

#[tracing::instrument(skip_all)]
pub async fn list_remarks(
    parameters: RemarksPageParameters,
    repository: &impl ListRemarks,
) -> ApplicationResult<Page<Remark>> {
    repository.list_remarks(parameters).await
}

#[tracing::instrument(skip_all)]
pub async fn update_remark(
    id: Uuid,
    changes: RemarkChanges,
    repository: &(impl UpdateRemark + GetRemark),
) -> ApplicationResult<Remark> {
    if changes.is_empty() {
        return Err(ApplicationError::invalid_argument("no remark changes provided"));
    }

    let mut remark = repository.get_remark(id).await?;

    let RemarkChanges { essence, tags } = changes;

    if let Some(essence) = essence {
        remark.set_essence(RemarkEssence::new(essence)?);
    }

    if let Some(tags) = tags {
        let tags = tags.into_iter().map(TagTitle::new).collect::<ApplicationResult<Vec<TagTitle>>>()?;
        remark.set_tags(tags);
    }

    repository.update_remark(&remark).await?;

    Ok(remark)
}

impl NewRemark {
    fn new(attributes: NewRemarkAttributes) -> ApplicationResult<Self> {
        let NewRemarkAttributes { essence, tags } = attributes;

        Ok(NewRemark {
            essence: RemarkEssence::new(essence)?,
            tags: tags.into_iter().map(|tag| TagTitle::new(tag)).collect::<ApplicationResult<Vec<TagTitle>>>()?,
        })
    }
}

impl RemarkChanges {
    fn is_empty(&self) -> bool {
        self.essence.is_none() && self.tags.is_none()
    }
}

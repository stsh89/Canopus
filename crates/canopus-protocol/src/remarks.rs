use crate::{Result, tags::Tag};
use chrono::{DateTime, Utc};
use std::future::Future;
use uuid::Uuid;

pub struct Remark {
    id: Uuid,
    essence: RemarkEssence,
    tags: Vec<Tag>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct RemarkAttributes {
    pub id: Uuid,
    pub essence: String,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct RemarkEssence(String);

pub struct NewRemark {
    pub essence: String,
    pub tags: Vec<String>,
}

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

impl RemarkUpdates {
    pub fn is_empty(&self) -> bool {
        self.add_tags.is_empty() && self.remove_tags.is_empty() && self.essence.is_none()
    }
}

#[derive(Default)]
pub struct RemarksListingParameters {
    pub pagination_token: Option<String>,
}

pub trait DeleteRemark {
    fn delete_remark(&self, id: Uuid) -> impl Future<Output = Result<()>>;
}

pub trait GetRemark {
    fn get_remark(&self, id: Uuid) -> impl Future<Output = Result<Remark>>;
}

pub trait InsertRemark {
    fn insert_remark(&self, new_remark: NewRemark) -> impl Future<Output = Result<Uuid>>;
}

pub trait UpdateRemark {
    fn update_remark(&self, parameters: RemarkUpdates) -> impl Future<Output = Result<()>>;
}

pub trait ListRemarks {
    fn list_remarks(
        &self,
        listing_parameters: RemarksListingParameters,
    ) -> impl Future<Output = Result<RemarksListing>>;
}

impl Remark {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn essence(&self) -> &RemarkEssence {
        &self.essence
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new(attributes: RemarkAttributes) -> Self {
        let RemarkAttributes {
            id,
            essence,
            tags,
            created_at,
            updated_at,
        } = attributes;

        Remark {
            id,
            essence: RemarkEssence(essence),
            tags,
            created_at,
            updated_at,
        }
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) {
        self.tags = tags;
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub async fn create_remark(new_remark: NewRemark, repository: &impl InsertRemark) -> Result<Uuid> {
    let NewRemark { essence, tags } = new_remark;

    let new_remark = NewRemark {
        essence: sanitize_essence(essence),
        tags: tags.into_iter().map(sanitize_tag).collect(),
    };

    repository.insert_remark(new_remark).await
}

pub async fn delete_remark(id: Uuid, repository: &impl DeleteRemark) -> Result<()> {
    repository.delete_remark(id).await?;

    Ok(())
}

pub async fn get_remark(id: Uuid, repository: &impl GetRemark) -> Result<Remark> {
    repository.get_remark(id).await
}

pub async fn list_remarks(
    parameters: RemarksListingParameters,
    repository: &impl ListRemarks,
) -> Result<RemarksListing> {
    repository.list_remarks(parameters).await
}

pub async fn update_remark(
    parameters: RemarkUpdates,
    repository: &impl UpdateRemark,
) -> Result<()> {
    repository.update_remark(parameters).await
}

fn sanitize_essence(essence: String) -> String {
    essence.trim().to_string()
}

fn sanitize_tag(tag: String) -> String {
    tag.trim().to_string()
}

impl std::ops::Deref for RemarkEssence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
